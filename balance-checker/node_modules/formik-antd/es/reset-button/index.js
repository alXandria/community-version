import { __rest } from "tslib";
import { Button } from 'antd';
import { Field } from 'formik';
import * as React from 'react';
export const ResetButton = (_a) => {
    var { children, onClick } = _a, restProps = __rest(_a, ["children", "onClick"]);
    return (React.createElement(Field, null, ({ form: { resetForm, dirty } }) => (React.createElement(Button, Object.assign({ onClick: (event) => {
            resetForm();
            onClick && onClick(event);
        }, disabled: !dirty, type: 'dashed' }, restProps), children))));
};
export default ResetButton;
//# sourceMappingURL=index.js.map