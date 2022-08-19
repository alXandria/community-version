import { __rest } from "tslib";
import { Switch as $Switch } from 'antd';
import * as React from 'react';
import Field from '../field';
export const Switch = (_a) => {
    var { name, validate, fast, onChange } = _a, restProps = __rest(_a, ["name", "validate", "fast", "onChange"]);
    return (React.createElement(Field, { name: name, validate: validate, fast: fast }, ({ field: { value }, form: { setFieldValue, setFieldTouched }, }) => (React.createElement($Switch, Object.assign({ checked: value, onChange: (checked, event) => {
            setFieldValue(name, checked);
            setFieldTouched(name, true, false);
            onChange && onChange(checked, event);
        } }, restProps)))));
};
export default Switch;
//# sourceMappingURL=index.js.map