import { __rest } from "tslib";
import { Select as $Select } from 'antd';
import * as React from 'react';
import Field from '../field';
// declare class Select<ValueType extends SelectValue = SelectValue> extends React.Component<SelectProps<ValueType>> {
export const Select = (_a) => {
    var { name, validate, fast, children, onChange, onBlur } = _a, restProps = __rest(_a, ["name", "validate", "fast", "children", "onChange", "onBlur"]);
    return (React.createElement(Field, { name: name, validate: validate, fast: fast }, ({ field: { value }, form: { setFieldValue, setFieldTouched }, }) => (React.createElement($Select, Object.assign({ onChange: (value, option) => {
            setFieldValue(name, value);
            onChange && onChange(value, option);
        }, onBlur: (value) => {
            setFieldTouched(name);
            onBlur && onBlur(value);
        }, 
        // setting undefined will show the placeholder
        value: value === '' || value === null ? undefined : value }, restProps), children))));
};
export default Select;
Select.Option = $Select.Option;
Select.OptGroup = $Select.OptGroup;
//# sourceMappingURL=index.js.map