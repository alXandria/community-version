import {Login as LoginDesign} from "@cosmicdapp/design";
import React from "react";
import {config} from "../../../config";
import {pathBalance} from "../../paths";
import cosmWasmLogo from "./assets/cosmWasmLogo.svg";

export function Login(): JSX.Element {
  return (
    <LoginDesign
      pathAfterLogin={pathBalance}
      appName="Balance checker"
      appLogo={cosmWasmLogo}
      config={config}
    />
  );
}
