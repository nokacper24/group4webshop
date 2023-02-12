import React from "react";
import DescriptionRow, { ProductPageRow, RowItem } from "./DescriptionRow";
import Gallery from "./gallery/Gallery";
import { SlidesProps } from "./gallery/Gallery";
import { ParagraphSlide } from "./gallery/ParagraphSlide";
import { SlideType } from "./gallery/SlideTypes";

let counter = 0;

export type State = {
  rows: ProductPageRow[];
};

export default function ProductPage() {
  let rowTextItem: RowItem = {
    title: "Lorem",
    content:
      "Lorem ipsum dolor sit amet consectetur adipisicing elit. Totam vitae tempora tempore dolore, porro reprehenderit suscipit, id sapiente molestias delectus alias saepe a doloribus est enim. Tenetur dolore sapiente eaque.",
    isTextNotImage: true,
  };
  let rowImageItem: RowItem = {
    title: undefined,
    content: "https://unsplash.it/300/200",
    isTextNotImage: false,
  };
  const row_data = [
    { item1: rowTextItem, item2: rowImageItem, textToLeft: true },
    { item1: rowImageItem, item2: rowTextItem, textToLeft: false },
  ];
  let state: State = {
    rows: [],
  };
  row_data.forEach((row) => {
    let newRow: ProductPageRow = {
      props: {
        item1: row.item1,
        item2: row.item2,
        textToLeft: row.textToLeft,
      },
    };
    state.rows.push(newRow);
  });
  let testimonial: SlidesProps = {
    slides: [
      {
        id: "slide1",
        mainContent:
          "Lorem ipsum, dolor sit amet consectetur adipisicing elit. Non perferendis reiciendis consectetur beatae commodi sapiente fugiat doloribus nam cupiditate vel distinctio, ducimus repudiandae iure tempore ipsam et sequi molestiae exercitationem!",
        reviewerProfile: {
          picturePath: "https://picsum.photos/100",
          name: "Joe Kerr",
          title: "Professional Clown",
        },
        slideType: SlideType.PARAGRAPH,
      },
    ],
  };
  return (
    <React.Fragment>
      <section className="banner">
        <div className="banner-inner">
          <div className="banner-highlight">
            <h1 className="banner-title banner-element hero-title">
              Product Name
            </h1>
            <button className="banner-element hero-button">Buy license</button>
          </div>
        </div>
      </section>
      <hr></hr>
      <section className="product-description">
        {state.rows.map((productPageRow) => (
          <DescriptionRow
            key={assignUniqueKey("row")}
            props={productPageRow.props}
          />
        ))}
      </section>
      <section className="gallery-wrapper">
        <h2>Testimonials</h2>
        <Gallery slides={testimonial.slides} />
      </section>
    </React.Fragment>
  );
}
/**
 * When you have multiple items with the same key, this adds an unique ID behind the key
 * and returns it.
 *
 * @param base the string you want an unique ID added to
 * @returns the string modified to include the ID
 */
function assignUniqueKey(base: string) {
  counter++;
  console.log(base + counter);
  return base + counter;
}
