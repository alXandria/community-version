import { Field } from 'formik';
import * as React from 'react';
export const isDevelopmentMode = () => !process.env.NODE_ENV || process.env.NODE_ENV === 'development';
export const FormikDebug = (props) => isDevelopmentMode() ? (React.createElement("pre", { style: Object.assign({ padding: 15 }, props) },
    React.createElement(Field, null, ({ form }) => JSON.stringify(form, null, 2)))) : null;
export default FormikDebug;
//# sourceMappingURL=index.js.map