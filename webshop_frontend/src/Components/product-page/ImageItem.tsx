import { RowItem } from "./DescriptionRow";

let baseUrl = import.meta.env.VITE_URL + ":" + import.meta.env.VITE_PORT;
// check if we are in production mode
if (import.meta.env.PROD) {
  baseUrl = "";
}

/**
 * Creates an image in a row
 *
 * @param rowItem image based RowItem
 * @returns JSX element
 */
export const ImageItem = (rowItem: RowItem | undefined) => {
  let item: JSX.Element;
  if (rowItem === undefined || rowItem.content === undefined) {
    item = <div className="empty-item"></div>;
  } else {
    item = <img src={baseUrl + rowItem.content} alt="" className="row-image" />;
  }
  return item;
};
