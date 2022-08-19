import {CW20, nativeCoinToDisplay, useAccount, useError, useSdk} from "@cosmicdapp/logic";
import {Coin, coins} from "@cosmjs/launchpad";
import {Decimal} from "@cosmjs/math";
import {Divider, Typography} from "antd";
import React, {useEffect, useState} from "react";
import {config} from "../../../../../config";
import {TokenItem, TokenStack} from "./style";

const {Text} = Typography;

interface
TokenListProps
{
  readonly
  contractAddress: string;
}

export function TokenList({contractAddress}: TokenListProps): JSX.Element {
  const {setError, clearError} = useError();
  const {getClient} = useSdk();
  const {account} = useAccount();

  const [balance, setBalance] = useState < readonly
  Coin[] > ([]);
  const [decimals, setDecimals] = useState < number > ();

  useEffect(() => {
    if (!contractAddress) {
      setBalance(account.balance);
      setDecimals(undefined);
      clearError();
      return;
    }

    const client = getClient();

    (async function updateBalance() {
      try {
        const contract = await client.getContract(contractAddress);
        const cw20Contract = CW20(client).use(contract.address);
        const [{symbol: denom, decimals}, balance] = await Promise.all([
          cw20Contract.tokenInfo(),
          cw20Contract.balance(),
        ]);
        const amount = parseInt(balance, 10);

        setBalance(coins(amount, denom));
        setDecimals(decimals);
        clearError();
      } catch {
        setError("No contract found in that address");
        setBalance([]);
        setDecimals(undefined);
      }
    })();
  }, [account.balance, getClient, contractAddress, clearError, setError]);

  function getCoinToDisplay(coin: Coin): Coin {
    if (contractAddress && decimals) {
      const amountFromDecimal = Decimal.fromAtomics(coin.amount, decimals).toString();
      return {denom: coin.denom, amount: amountFromDecimal};
    }

    return nativeCoinToDisplay(coin, config.coinMap);
  }

  const isCw20Token = contractAddress && decimals !== undefined;
  const isNativeToken = !contractAddress && decimals === undefined;
  const showTokens = isCw20Token || isNativeToken;

  return (
    showTokens && (
      <TokenStack>
        {balance.map((token, index) => {
          const {denom, amount} = getCoinToDisplay(token);

          return (
            <React.Fragment key={token.denom}>
              {index > 0 && <Divider/>}
              <TokenItem>
                <Text>{denom}</Text>
                <Text>{amount !== "0" ? amount : "No tokens"}</Text>
              </TokenItem>
            </React.Fragment>
          );
        })}
      </TokenStack>
    )
  );
}