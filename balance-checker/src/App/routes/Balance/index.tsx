import {PageLayout, YourAccount} from "@cosmicdapp/design";
import {useError} from "@cosmicdapp/logic";
import {Typography} from "antd";
import React, {useState} from "react";
import {FormCheckBalance} from "./components/FormCheckBalance";
import {TokenList} from "./components/TokenList";
import {ErrorText, MainStack} from "./style";

const {Title} = Typography;

export function Balance(): JSX.Element {
  const {error} = useError();
  const [contractAddress, setContractAddress] = useState();

  return (
    <PageLayout>
      <MainStack>
        <Title>Balance</Title>
        <YourAccount hideTitle hideBalance/>
        <FormCheckBalance setContractAddress={setContractAddress}/>
        {error && <ErrorText>{error}</ErrorText>}
        <TokenList contractAddress={contractAddress}/>
      </MainStack>
    </PageLayout>
  );
}