import DescriptionRow, { ProductPageRow, RowItem } from "./DescriptionRow";

export type State = {
    rows: ProductPageRow[];
}

export default function ProductPage() {
    let rowTextItem: RowItem = {
        title: "Lorem",
        content: "Lorem ipsum dolor sit amet consectetur adipisicing elit. Totam vitae tempora tempore dolore, porro reprehenderit suscipit, id sapiente molestias delectus alias saepe a doloribus est enim. Tenetur dolore sapiente eaque.",
        isTextNotImage: true,
    }
    let rowImageItem: RowItem = {
        title: undefined,
        content: 'https://unsplash.it/300/200',
        isTextNotImage: false,
    }
    const row_data = [
        {item1: rowTextItem, item2: rowImageItem, textToLeft: true},
        {item1: rowImageItem, item2: rowTextItem, textToLeft: false},
    ]
    let state: State = {
        rows: [],
    }
    row_data.forEach((row) => {
        let newRow: ProductPageRow= {
            props: row, 
        }
        state.rows.push(newRow);
    });
    return (
        <section className="container">
            <h1 className="hero-title">Profile</h1>
            {
                state.rows.map((productPageRow) => (
                    <DescriptionRow
                        key={assignUniqueKey("")}
                        props={productPageRow.props} />
                        
                ))
            }
        </section>
    );
}

let counter = 0;
function assignUniqueKey(base: string) {
    counter++;
    return base + counter;
}