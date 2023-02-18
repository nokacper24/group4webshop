import React from "react";
import { Link, Route, Routes } from "react-router-dom";
import DescriptionRow, { ProductPageRow, RowItem } from "./DescriptionRow";
import Gallery from "./gallery/Gallery";
import { GalleryProps } from "./gallery/Gallery";
import { SlideType } from "./gallery/SlideTypes";
import PurchaseLicense from "./PurchaseLicense";

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
  const rowData = [
    { item1: rowTextItem, item2: rowImageItem, textToLeft: true },
    { item1: rowImageItem, item2: rowTextItem, textToLeft: false },
  ];
  let state: State = {
    rows: [],
  };
  rowData.forEach((row) => {
    let newRow: ProductPageRow = {
      props: {
        item1: row.item1,
        item2: row.item2,
        textToLeft: row.textToLeft,
      },
    };
    state.rows.push(newRow);
  });
  let testimonial: GalleryProps = {
    galleryName: "testGallery",
    slides: [
      {
        slideId: "slide1",
        mainContent:
          "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nam sapien orci, varius quis mauris a, blandit imperdiet tellus. Donec a cursus leo. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Cras tincidunt ex vel libero porttitor, quis vulputate mauris condimentum. Mauris blandit purus at mauris fringilla pretium. Donec pharetra justo in ultricies accumsan. Duis ullamcorper condimentum porttitor. Nunc pellentesque vestibulum est, et dictum metus pellentesque nec. Morbi luctus turpis vitae facilisis tristique. Duis sed posuere magna. Aliquam sodales, turpis in consequat tristique, nibh odio luctus libero, quis fringilla metus turpis vitae lorem.",
        reviewerProfile: {
          picturePath: "https://picsum.photos/100",
          name: "Joe Kerr",
          title: "Professional Clown",
        },
        slideType: SlideType.PARAGRAPH,
      },
      {
        slideId: "slide2",
        mainContent:
          "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nunc in tellus nibh. Interdum et malesuada fames ac ante ipsum primis in faucibus. Quisque a malesuada nunc, quis aliquam ante. Nulla elementum rutrum magna quis condimentum. Integer nunc enim, semper ut sodales eu, blandit quis leo. Ut blandit mollis est, sit amet ultrices ligula facilisis sed. Morbi non est rutrum, aliquet metus non, iaculis nisl. Donec nec magna hendrerit, elementum elit sit amet, sodales tortor. Pellentesque nulla orci, tincidunt vel lacinia condimentum, euismod ut mi. Integer tristique metus a eros luctus, ac sollicitudin dui iaculis. Vestibulum iaculis consequat dui a lacinia. Fusce id leo eu eros fringilla efficitur id vel nisi.",
        reviewerProfile: {
          picturePath: "https://picsum.photos/100",
          name: "Bat mann",
          title: "Genius acrobat",
        },
        slideType: SlideType.PARAGRAPH,
      },
      {
        slideId: "slide3",
        mainContent:
          "Phasellus id nibh eget justo blandit rhoncus et ut libero. Nunc ullamcorper, elit id interdum faucibus, leo ipsum tristique libero, nec varius ante nunc ut purus. In blandit in odio vel convallis. Curabitur non elementum elit, sed vestibulum dui. Phasellus eu dolor magna. Maecenas viverra orci id pellentesque auctor. Duis eu efficitur nunc. Proin ut interdum est. Proin sed volutpat tellus, venenatis dictum augue. Cras ante enim, convallis quis enim eget, scelerisque aliquam nibh. Nunc id sagittis dolor. Praesent luctus et felis vitae laoreet. Quisque ultricies sapien risus, in faucibus odio faucibus non. Aliquam erat volutpat. Proin consectetur blandit ex in aliquam.",
        reviewerProfile: {
          picturePath: "https://picsum.photos/100",
          name: "Pene Guin",
          title: "Animal",
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
            <Link to="purchase-license">
              <button className="banner-element hero-button">
                Buy license
              </button>
            </Link>
          </div>
        </div>
      </section>
      <hr></hr>
      <section className="product-description container">
        {state.rows.map((productPageRow) => (
          <DescriptionRow
            key={assignUniqueKey("row")}
            props={productPageRow.props}
          />
        ))}
      </section>
      { testimonial.slides.length > 0 &&
        <section className="gallery-wrapper">
          <div className="container">
            <h2 className="testimonial-title">Testimonials</h2>
            <Gallery
              slides={testimonial.slides}
              galleryName={testimonial.galleryName}
            />
          </div>
        </section>
      }
      <section className="container">
        <Link to="purchase-license">
          <button className="banner-element hero-button">Buy license</button>
        </Link>
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
