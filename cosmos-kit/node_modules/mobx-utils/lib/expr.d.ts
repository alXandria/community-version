/**
 *`expr` can be used to create temporary computed values inside computed values.
 * Nesting computed values is useful to create cheap computations in order to prevent expensive computations from needing to run.
 * In the following example the expression prevents that a component is rerender _each time_ the selection changes;
 * instead it will only rerenders when the current todo is (de)selected.
 *
 * `expr(func)` is an alias for `computed(func).get()`.
 * Please note that the function given to `expr` is evaluated _twice_ in the scenario that the overall expression value changes.
 * It is evaluated the first time when any observables it depends on change.
 * It is evaluated a second time when a change in its value triggers the outer computed or reaction to evaluate, which recreates and reevaluates the expression.
 *
 * In the following example, the expression prevents the `TodoView` component from being re-rendered if the selection changes elsewhere.
 * Instead, the component will only re-render when the relevant todo is (de)selected, which happens much less frequently.
 *
 * @example
 * const Todo = observer((props) => {
 *     const todo = props.todo
 *     const isSelected = mobxUtils.expr(() => props.viewState.selection === todo)
 * const TodoView = observer(({ todo, editorState }) => {
 *     const isSelected = mobxUtils.expr(() => editorState.selection === todo)
 *     return <div className={isSelected ? "todo todo-selected" : "todo"}>{todo.title}</div>
 * })
 */
export declare function expr<T>(expr: () => T): T;
