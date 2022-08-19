import { __rest } from "tslib";
import * as React from 'react';
import { Cascader as $Cascader } from 'antd';
import Field from '../field';
export const Cascader = (_a) => {
    var { name, validate, fast, onChange } = _a, restProps = __rest(_a, ["name", "validate", "fast", "onChange"]);
    return (React.createElement(Field, { name: name, validate: validate, fast: fast }, ({ field: { value }, form: { setFieldValue, setFieldTouched }, }) => (React.createElement($Cascader, Object.assign({ value: value, onChange: (value) => {
            setFieldValue(name, value);
            setFieldTouched(name, true, false);
            onChange && onChange(value);
        } }, restProps)))));
};
export default Cascader;
//# sourceMappingURL=index.js.map