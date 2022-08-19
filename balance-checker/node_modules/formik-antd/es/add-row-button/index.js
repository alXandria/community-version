import { __rest } from "tslib";
import * as React from 'react';
import { ArrayButton } from '../array-button';
export function AddRowButton(_a) {
    var { name, createNewRow } = _a, restProps = __rest(_a, ["name", "createNewRow"]);
    return (React.createElement(ArrayButton, Object.assign({ name: name }, restProps, { onClick: (array) => array.push(createNewRow()) })));
}
export default AddRowButton;
//# sourceMappingURL=index.js.map