import { ImageItem } from "./ImageItem";
import { TextItem } from "./TextItem";

export type ProductPageRow = {
  props: {
    item1: RowItem | undefined;
    item2: RowItem | undefined;
    textToLeft: boolean;
  };
};

export type RowItem = {
  title: string | undefined;
  content: string;
  isTextNotImage: boolean;
};

export default function DescriptionRow(row: ProductPageRow) {
  let rowElement: [JSX.Element | undefined, JSX.Element | undefined];

  if (row.props.item1?.isTextNotImage && !row.props.item2?.isTextNotImage) {
    //if item1 is text but not item2
    if (row.props.textToLeft) {
      rowElement = [TextItem(row.props.item1), ImageItem(row.props.item2)];
      console.log(
        "Item 1: " +
          row.props.item1.content +
          " Item 2: " +
          row.props.item2 +
          " functionID: 1"
      );
    } else {
      rowElement = [ImageItem(row.props.item2), TextItem(row.props.item1)];
      console.log(
        "Item 1: " +
          row.props.item1.content +
          " Item 2: " +
          row.props.item2 +
          " functionID: 2"
      );
    }
  } else if (
    !row.props.item1?.isTextNotImage &&
    row.props.item2?.isTextNotImage
  ) {
    //if item2 is text but item1 is not
    if (row.props.textToLeft) {
      rowElement = [TextItem(row.props.item2), ImageItem(row.props.item1)];
      console.log(
        "Item 1: " + " Item 2: " + row.props.item2 + " functionID: 3"
      );
    } else {
      rowElement = [ImageItem(row.props.item1), TextItem(row.props.item2)];
      console.log(
        "Item 1: " + " Item 2: " + row.props.item2 + " functionID: 4"
      );
    }
  } else {
    //if item1 and item2 is the same the order will be item1 > item2
    if (row.props.item1?.isTextNotImage && row.props.item2?.isTextNotImage) {
      rowElement = [TextItem(row.props.item1), TextItem(row.props.item2)];
      console.log(
        "Item 1: " +
          row.props.item1.content +
          " Item 2: " +
          row.props.item2 +
          " functionID: 5"
      );
    } else {
      rowElement = [ImageItem(row.props.item1), ImageItem(row.props.item2)];
      console.log(
        "Item 1: " + " Item 2: " + row.props.item2 + " functionID: 6"
      );
    }
  }
  return <div>{rowElement}</div>;
}
