import { ImageItem } from "./ImageItem";
import { TextItem } from "./TextItem";

/**
 * Object that symbolizes a row in the description of the product page
 */
export type ProductPageRow = {
  props: {
    item1: RowItem | undefined;
    item2: RowItem | undefined;
    textToLeft: boolean;
  };
};

/**
 * Object that symbolizes a item in a row in the description of the product page
 */
export type RowItem = {
  title: string | undefined;
  content: string;
  isTextNotImage: boolean;
};
/**
 * Properly creates the layout of a row based on the props of the row
 *
 * @param row Row to create layout for
 * @returns JSX element
 */
export default function DescriptionRow(row: ProductPageRow) {
  let rowElement: [string, JSX.Element | undefined, JSX.Element | undefined];

  if (row.props.item1?.isTextNotImage && !row.props.item2?.isTextNotImage) {
    //if item1 is text but not item2
    if (row.props.textToLeft) {
      rowElement = [
        "rowID",
        TextItem(row.props.item1),
        ImageItem(row.props.item2),
      ];
    } else {
      rowElement = [
        "rowID",
        ImageItem(row.props.item2),
        TextItem(row.props.item1),
      ];
    }
  } else if (
    !row.props.item1?.isTextNotImage &&
    row.props.item2?.isTextNotImage
  ) {
    //if item2 is text but item1 is not
    if (row.props.textToLeft) {
      rowElement = [
        "rowID",
        TextItem(row.props.item2),
        ImageItem(row.props.item1),
      ];
    } else {
      rowElement = [
        "rowID",
        ImageItem(row.props.item1),
        TextItem(row.props.item2),
      ];
    }
  } else {
    //if item1 and item2 is the same the order will be item1 > item2
    if (row.props.item1?.isTextNotImage && row.props.item2?.isTextNotImage) {
      rowElement = [
        "rowID",
        TextItem(row.props.item1),
        TextItem(row.props.item2),
      ];
    } else {
      rowElement = [
        "rowID",
        ImageItem(row.props.item1),
        ImageItem(row.props.item2),
      ];
    }
  }
  return (
    <div className="product-description-row" key={rowElement[0]}>
      {
        // remove first element in array (id) and map the rest
        rowElement.slice(1).map((item, index) => {
          if (item) {
            return (
              <div className="product-description-row-item" key={index}>
                {item}
              </div>
            );
          }
        })
      }
    </div>
  );
}
