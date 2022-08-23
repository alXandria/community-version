var __extends = (this && this.__extends) || (function () {
    var extendStatics = function (d, b) {
        extendStatics = Object.setPrototypeOf ||
            ({ __proto__: [] } instanceof Array && function (d, b) { d.__proto__ = b; }) ||
            function (d, b) { for (var p in b) if (Object.prototype.hasOwnProperty.call(b, p)) d[p] = b[p]; };
        return extendStatics(d, b);
    };
    return function (d, b) {
        extendStatics(d, b);
        function __() { this.constructor = d; }
        d.prototype = b === null ? Object.create(b) : (__.prototype = b.prototype, new __());
    };
})();
import { observable, reaction, observe, transaction, ObservableMap, } from "mobx";
/**
 * Reactively sorts a base observable array into multiple observable arrays based on the value of a
 * `groupBy: (item: T) => G` function.
 *
 * This observes the individual computed groupBy values and only updates the source and dest arrays
 * when there is an actual change, so this is far more efficient than, for example
 * `base.filter(i => groupBy(i) === 'we')`. Call #dispose() to stop tracking.
 *
 * No guarantees are made about the order of items in the grouped arrays.
 *
 * The resulting map of arrays is read-only. clear(), set(), delete() are not supported and
 * modifying the group arrays will lead to undefined behavior.
 *
 * NB: ObservableGroupMap relies on `Symbol`s. If you are targeting a platform which doesn't
 * support these natively, you will need to provide a polyfill.
 *
 * @param {array} base The array to sort into groups.
 * @param {function} groupBy The function used for grouping.
 * @param options Object with properties:
 *  `name`: Debug name of this ObservableGroupMap.
 *  `keyToName`: Function to create the debug names of the observable group arrays.
 *
 * @example
 * const slices = observable([
 *     { day: "mo", hours: 12 },
 *     { day: "tu", hours: 2 },
 * ])
 * const slicesByDay = new ObservableGroupMap(slices, (slice) => slice.day)
 * autorun(() => console.log(
 *     slicesByDay.get("mo")?.length ?? 0,
 *     slicesByDay.get("we"))) // outputs 1, undefined
 * slices[0].day = "we" // outputs 0, [{ day: "we", hours: 12 }]
 */
var ObservableGroupMap = /** @class */ (function (_super) {
    __extends(ObservableGroupMap, _super);
    function ObservableGroupMap(base, groupBy, _a) {
        var _b = _a === void 0 ? {} : _a, _c = _b.name, name = _c === void 0 ? "ogm" + ((Math.random() * 1000) | 0) : _c, _d = _b.keyToName, keyToName = _d === void 0 ? function (x) { return "" + x; } : _d;
        var _this = _super.call(this) || this;
        /**
         * Base observable array which is being sorted into groups.
         */
        Object.defineProperty(_this, "_base", {
            enumerable: true,
            configurable: true,
            writable: true,
            value: void 0
        });
        /**
         * The ObservableGroupMap needs to track some state per-item. This is the name/symbol of the
         * property used to attach the state.
         */
        Object.defineProperty(_this, "_ogmInfoKey", {
            enumerable: true,
            configurable: true,
            writable: true,
            value: void 0
        });
        /**
         * The function used to group the items.
         */
        Object.defineProperty(_this, "_groupBy", {
            enumerable: true,
            configurable: true,
            writable: true,
            value: void 0
        });
        /**
         * This function is used to generate the mobx debug names of the observable group arrays.
         */
        Object.defineProperty(_this, "_keyToName", {
            enumerable: true,
            configurable: true,
            writable: true,
            value: void 0
        });
        Object.defineProperty(_this, "_disposeBaseObserver", {
            enumerable: true,
            configurable: true,
            writable: true,
            value: void 0
        });
        _this._keyToName = keyToName;
        _this._groupBy = groupBy;
        _this._ogmInfoKey = Symbol("ogmInfo" + name);
        _this._base = base;
        for (var i = 0; i < base.length; i++) {
            _this._addItem(base[i]);
        }
        _this._disposeBaseObserver = observe(_this._base, function (change) {
            if ("splice" === change.type) {
                transaction(function () {
                    for (var _i = 0, _a = change.removed; _i < _a.length; _i++) {
                        var removed = _a[_i];
                        _this._removeItem(removed);
                    }
                    for (var _b = 0, _c = change.added; _b < _c.length; _b++) {
                        var added = _c[_b];
                        _this._addItem(added);
                    }
                });
            }
            else if ("update" === change.type) {
                transaction(function () {
                    _this._removeItem(change.oldValue);
                    _this._addItem(change.newValue);
                });
            }
            else {
                throw new Error("illegal state");
            }
        });
        return _this;
    }
    Object.defineProperty(ObservableGroupMap.prototype, "clear", {
        enumerable: false,
        configurable: true,
        writable: true,
        value: function () {
            throw new Error("not supported");
        }
    });
    Object.defineProperty(ObservableGroupMap.prototype, "delete", {
        enumerable: false,
        configurable: true,
        writable: true,
        value: function (_key) {
            throw new Error("not supported");
        }
    });
    Object.defineProperty(ObservableGroupMap.prototype, "set", {
        enumerable: false,
        configurable: true,
        writable: true,
        value: function (_key, _value) {
            throw new Error("not supported");
        }
    });
    /**
     * Disposes all observers created during construction and removes state added to base array
     * items.
     */
    Object.defineProperty(ObservableGroupMap.prototype, "dispose", {
        enumerable: false,
        configurable: true,
        writable: true,
        value: function () {
            this._disposeBaseObserver();
            for (var i = 0; i < this._base.length; i++) {
                var item = this._base[i];
                var grouperItemInfo = item[this._ogmInfoKey];
                grouperItemInfo.reaction();
                delete item[this._ogmInfoKey];
            }
        }
    });
    Object.defineProperty(ObservableGroupMap.prototype, "_getGroupArr", {
        enumerable: false,
        configurable: true,
        writable: true,
        value: function (key) {
            var result = _super.prototype.get.call(this, key);
            if (undefined === result) {
                result = observable([], { name: "GroupArray[" + this._keyToName(key) + "]", deep: false });
                _super.prototype.set.call(this, key, result);
            }
            return result;
        }
    });
    Object.defineProperty(ObservableGroupMap.prototype, "_removeFromGroupArr", {
        enumerable: false,
        configurable: true,
        writable: true,
        value: function (key, itemIndex) {
            var arr = _super.prototype.get.call(this, key);
            if (1 === arr.length) {
                _super.prototype.delete.call(this, key);
            }
            else if (itemIndex === arr.length - 1) {
                // last position in array
                arr.length--;
            }
            else {
                arr[itemIndex] = arr[arr.length - 1];
                arr[itemIndex][this._ogmInfoKey].groupArrIndex = itemIndex;
                arr.length--;
            }
        }
    });
    Object.defineProperty(ObservableGroupMap.prototype, "_addItem", {
        enumerable: false,
        configurable: true,
        writable: true,
        value: function (item) {
            var _this = this;
            var groupByValue = this._groupBy(item);
            var groupArr = this._getGroupArr(groupByValue);
            var value = {
                groupByValue: groupByValue,
                groupArrIndex: groupArr.length,
                reaction: reaction(function () { return _this._groupBy(item); }, function (newGroupByValue, _r) {
                    var grouperItemInfo = item[_this._ogmInfoKey];
                    _this._removeFromGroupArr(grouperItemInfo.groupByValue, grouperItemInfo.groupArrIndex);
                    var newGroupArr = _this._getGroupArr(newGroupByValue);
                    var newGroupArrIndex = newGroupArr.length;
                    newGroupArr.push(item);
                    grouperItemInfo.groupByValue = newGroupByValue;
                    grouperItemInfo.groupArrIndex = newGroupArrIndex;
                }),
            };
            Object.defineProperty(item, this._ogmInfoKey, {
                configurable: true,
                enumerable: false,
                value: value,
            });
            groupArr.push(item);
        }
    });
    Object.defineProperty(ObservableGroupMap.prototype, "_removeItem", {
        enumerable: false,
        configurable: true,
        writable: true,
        value: function (item) {
            var grouperItemInfo = item[this._ogmInfoKey];
            this._removeFromGroupArr(grouperItemInfo.groupByValue, grouperItemInfo.groupArrIndex);
            grouperItemInfo.reaction();
            delete item[this._ogmInfoKey];
        }
    });
    return ObservableGroupMap;
}(ObservableMap));
export { ObservableGroupMap };
