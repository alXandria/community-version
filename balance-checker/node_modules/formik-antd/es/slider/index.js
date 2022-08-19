import { __rest } from "tslib";
import { Slider as $Slider } from 'antd';
import * as React from 'react';
import Field from '../field';
export const Slider = (_a) => {
    var { name, validate, fast, onChange } = _a, restProps = __rest(_a, ["name", "validate", "fast", "onChange"]);
    return (React.createElement(Field, { name: name, validate: validate, fast: fast }, ({ field: { value }, form: { setFieldValue, setFieldTouched }, }) => (React.createElement($Slider, Object.assign({ value: value, onChange: (value) => {
            setFieldValue(name, value != null ? value.valueOf() : value);
            setFieldTouched(name, true, false);
            onChange && onChange(value);
        } }, restProps)))));
};
export default Slider;
//# sourceMappingURL=index.js.map