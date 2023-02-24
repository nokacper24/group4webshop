import { RowItem } from "./DescriptionRow";

/**
 * Creates a paragraph in a row
 *
 * @param rowItem text based RowItem
 * @returns JSX element
 */
export const TextItem = (rowItem: RowItem | undefined) => {
  let item: JSX.Element;
  if (rowItem === undefined || rowItem.content === undefined) {
    item = <div className="empty-item"></div>; //if rowItem is undefined, we look at it as an empty slot.
  } else {
    item = (
      <div className="text-item-wrapper">
        <h2 className="row-title">{rowItem.title}</h2>
        <p className="row-text">{rowItem.content}</p>
      </div>
    );
  }
  return item;
};
