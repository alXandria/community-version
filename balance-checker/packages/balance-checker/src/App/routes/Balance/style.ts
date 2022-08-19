import {Stack} from "@cosmicdapp/design";
import {Typography} from "antd";
import styled from "styled-components";

const {Text} = Typography;

export const MainStack = styled(Stack)`
  & > * {
    --gap: var(--s4);
  }

  h1 {
    margin: 0;
  }

  .ant-form {
    margin-top: var(--gap);
  }
`;

export const ErrorText = styled(Text)`
  color: var(--color-red);
`;