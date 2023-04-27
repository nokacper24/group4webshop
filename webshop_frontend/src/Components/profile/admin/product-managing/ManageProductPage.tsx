import { useParams } from "react-router-dom";
import AccordionTable from "./Accordion/AccordionTable";
import HeaderEditPopup from "./Edit-popups/HeaderEditPopup";
import RowEditPopup from "./Edit-popups/RowEditPopup";
import { useEffect, useRef } from "react";
import TestimonialPopup from "./Edit-popups/TestimonialPopup";

export default function ManageProductPage() {
  let { productId } = useParams();

  const productName = useRef(null);
  const productPrice = useRef(null);
  const productImage = useRef(null);
  const productDescription = useRef(null);

  let createState = productId !== undefined;

  useEffect(() => {
    if (!createState) {
    } else {
      productId = "placeholder_id";
    }
  });
  return (
    <>
      <HeaderEditPopup></HeaderEditPopup>
      <RowEditPopup></RowEditPopup>
      <TestimonialPopup product_id={productId!}></TestimonialPopup>
      <section className="container">
        <h2>Manage product</h2>
        <form>
          <label htmlFor="product-name">Product name:</label>
          <input
            type="text"
            id="product-name"
            name="product-name"
            ref={productName}
          />
          <label htmlFor="product-price">Product price:</label>
          <div>
            <input
              type="number"
              id="product-price"
              name="product-price"
              ref={productPrice}
            />
            <p>kr</p>
          </div>
          <label htmlFor="product-image">Upload header image</label>
          <input
            type="file"
            id="product-image"
            name="product-image"
            accept="image/png, image/jpeg, image/webp"
            ref={productImage}
          />
          <label htmlFor="product-description">Description:</label>
          <textarea
            id="product-description"
            name="product-description"
            rows={10}
            cols={50}
            ref={productDescription}
          />
        </form>
      </section>
      <section className="accordion-wrapper container">
        <AccordionTable></AccordionTable>
      </section>
      <section className="container">
        <iframe src=""></iframe>
      </section>
      <section className="button-container">
        <button className="default-button small-button bg-danger">
          Delete product permanently
        </button>
        <button className="default-button small-button">Save</button>
      </section>
    </>
  );
}
