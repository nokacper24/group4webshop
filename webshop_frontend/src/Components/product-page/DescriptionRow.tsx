export type ProductPageRow = {
    props: {
        item1: RowItem | undefined,
        item2: RowItem | undefined,
        textToLeft: boolean,
    }
}
export type RowItem = {
    title: string | undefined,
    content: string,
    isTextNotImage: boolean
}
export default function DescriptionRow (row: ProductPageRow) {
    return (
    );
