import { __rest } from "tslib";
import * as React from 'react';
import { ArrayButton } from '../array-button';
export function RemoveRowButton(_a) {
    var { name, index } = _a, restProps = __rest(_a, ["name", "index"]);
    return (React.createElement(ArrayButton, Object.assign({ name: name }, restProps, { onClick: (array) => array.remove(index) })));
}
export default RemoveRowButton;
//# sourceMappingURL=index.js.map