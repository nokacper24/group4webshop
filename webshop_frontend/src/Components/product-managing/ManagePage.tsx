import { AccordionHeader } from "./Accordion/AccordionHeader";
import AccordionTable from "./Accordion/AccordionTable";
import HeaderEditPopup from "./Edit-popups/HeaderEditPopup";
import RowEditPopup from "./Edit-popups/RowEditPopup";

export default function ManagePage() {
  return (
    <>
      <HeaderEditPopup></HeaderEditPopup>
      <RowEditPopup
        image={false}
        title={undefined}
        content={undefined}
      ></RowEditPopup>
      <section className="container">
        <h2>Manage product</h2>
        <form>
          <label htmlFor="product-name">Product name:</label>
          <input type="text" id="product-name" name="product-name" />
          <label htmlFor="product-price">Product price:</label>
          <div>
            <input type="number" id="product-price" name="product-price" />
            <p>kr</p>
          </div>
          <label htmlFor="product-image">Upload header image</label>
          <input
            type="file"
            id="product-image"
            name="product-image"
            accept="image/png, image/jpeg, image/webp"
          />
          <label htmlFor="product-description">Description:</label>
          <textarea
            id="product-description"
            name="product-description"
            rows={10}
            cols={50}
          />
        </form>
      </section>
      <section className="accordion-wrapper container">
        <AccordionTable></AccordionTable>
      </section>
      <section className="container">
        <iframe src=""></iframe>
      </section>
      <section className="container">
        <button>Delete product permanently</button>
        <button>Save</button>
      </section>
    </>
  );
}
