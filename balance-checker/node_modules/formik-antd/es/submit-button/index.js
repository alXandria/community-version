import { __rest } from "tslib";
import { Button } from 'antd';
import { Field } from 'formik';
import * as React from 'react';
export const SubmitButton = (_a) => {
    var { children } = _a, restProps = __rest(_a, ["children"]);
    return (React.createElement(Field, null, ({ form: { isSubmitting, isValid, dirty, submitCount } }) => (React.createElement(Button, Object.assign({ loading: isSubmitting, type: 'primary', htmlType: 'submit', disabled: !isValid && (dirty || submitCount > 0) }, restProps), children))));
};
export default SubmitButton;
//# sourceMappingURL=index.js.map