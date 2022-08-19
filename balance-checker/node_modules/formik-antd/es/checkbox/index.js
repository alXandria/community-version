import { __rest } from "tslib";
import * as React from 'react';
import { Checkbox as $Checkbox } from 'antd';
import Field from '../field';
export const Checkbox = (_a) => {
    var { name, validate, fast, onChange } = _a, restProps = __rest(_a, ["name", "validate", "fast", "onChange"]);
    return (React.createElement(Field, { name: name, validate: validate, fast: fast }, ({ field: { value }, form: { setFieldValue, setFieldTouched }, }) => (React.createElement($Checkbox, Object.assign({ name: name, checked: value, onChange: (event) => {
            setFieldValue(name, event.target.checked);
            setFieldTouched(name, true, false);
            onChange && onChange(event);
        } }, restProps)))));
};
export default Checkbox;
Checkbox.Group = (_a) => {
    var { name, validate, onChange } = _a, restProps = __rest(_a, ["name", "validate", "onChange"]);
    return (React.createElement(Field, { name: name, validate: validate }, ({ field: { value }, form: { setFieldValue, setFieldTouched }, }) => (React.createElement($Checkbox.Group, Object.assign({ value: value, onChange: (value) => {
            setFieldValue(name, value);
            setFieldTouched(name, true, false);
            onChange && onChange(value);
        } }, restProps)))));
};
//# sourceMappingURL=index.js.map