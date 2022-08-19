import { __rest } from "tslib";
import { Radio as $Radio } from 'antd';
import * as React from 'react';
import Field from '../field';
export const Radio = (_a) => {
    var { name, validate, fast, onChange } = _a, restProps = __rest(_a, ["name", "validate", "fast", "onChange"]);
    return (React.createElement(Field, { name: name, validate: validate, fast: fast }, ({ field: { value }, form: { setFieldValue, setFieldTouched }, }) => (React.createElement($Radio, Object.assign({ value: value, onChange: (event) => {
            setFieldValue(name, event.target.value);
            setFieldTouched(name, true, false);
            onChange && onChange(event);
        } }, restProps)))));
};
export default Radio;
Radio.Group = (_a) => {
    var { name, validate, fast, onChange } = _a, restProps = __rest(_a, ["name", "validate", "fast", "onChange"]);
    return (React.createElement(Field, { name: name, validate: validate, fast: fast }, ({ field: { value }, form: { setFieldValue, setFieldTouched }, }) => (React.createElement($Radio.Group, Object.assign({ value: value, onChange: (event) => {
            setFieldValue(name, event.target.value);
            setFieldTouched(name, true, false);
            onChange && onChange(event);
        } }, restProps)))));
};
Radio.Button = $Radio.Button;
//# sourceMappingURL=index.js.map