import { useParams } from "react-router-dom";
import AccordionTable from "./Accordion/AccordionTable";
import HeaderEditPopup from "./Edit-popups/HeaderEditPopup";
import RowEditPopup from "./Edit-popups/RowEditPopup";
import { useEffect, useRef, useState } from "react";
import TestimonialPopup from "./Edit-popups/TestimonialPopup";
import {
  Description,
  Product,
  SimpleDescription,
  Testimonial,
} from "../../../../Interfaces";
import {
  fetchDescriptionComponents,
  fetchProduct,
  fetchTestimonials,
} from "../../../../ApiController";
import { AccordionSectionProps } from "./Accordion/AccordionSection";
import { ChangeType } from "./Accordion/ChangeTypes";

/**
 * Page for creating or editing a product page.
 *
 * @returns the React component for the ManageProductPage
 */
export default function ManageProductPage() {
  let { productId } = useParams();

  const productName = useRef<HTMLInputElement>(null);
  const productPrice = useRef<HTMLInputElement>(null);
  const productImage = useRef<HTMLInputElement>(null);
  const productDescription = useRef<HTMLTextAreaElement>(null);

  const [testimonials, setTestimonials] = useState<Testimonial[]>([]);
  const [sections, setSections] = useState<AccordionSectionProps[]>([]);
  const [productInfo, setProductInfo] = useState<Product>();

  let createState = productId === undefined;

  useEffect(() => {
    if (!createState) {
      initializeData();
    } else {
      productId = "placeholder_id";
    }
  }, []);

  useEffect(() => {
    assignProductInfo();
  }, [productInfo]);

  const initializeData = () => {
    fetchTestimonials(productId!).then((testimonials: Testimonial[]) =>
      setTestimonials(testimonials)
    );
    fetchDescriptionComponents(productId!).then((descriptions: Description[]) =>
      initializeSections(assignImageState(descriptions))
    );
    fetchProduct(productId!).then((product: Product) => {
      setProductInfo(product);
    });
  };

  const [priorityChanges, setPriorityChanges] = useState<number[]>(); //Section IDs that has had a swap change as value
  const [contentChanges, setContentChanges] = useState<
    Map<ChangeType, number[]>
  >(new Map()); //The type of change as key, list of IDs that has had that change as value

  useEffect(() => {
    // Sets up the map so that it can register changes
    setContentChanges((changes) => {
      for (let type in ChangeType) {
        changes.set(ChangeType[type as keyof typeof ChangeType], []);
      }
      return changes;
    });
  });

  /**
   * Registers changes to the content of the table. This is used to keep track of what has changed and what needs to be saved.
   *
   * @param id The ID of the section that has changed
   * @param change The type of change that has been made
   */
  const registerContentChange = (id: number, change: ChangeType) => {
    if (!contentChanges.get(change)?.includes(id)) {
      contentChanges.get(change)?.push(id);
    }
    if (change === ChangeType.Delete) {
      contentChanges
        .get(ChangeType.Edit)
        ?.filter((changeId) => changeId !== id);
      priorityChanges?.filter((changeId) => changeId !== id);
      setPriorityChanges(priorityChanges);
    } else if (change === ChangeType.Swap) {
      if (!priorityChanges?.includes(id)) {
        priorityChanges?.push(id);
        setPriorityChanges(priorityChanges);
      } else {
        //Since a section only can have two rows at a time, we can assume that if the section already is on the list, the rows are swapped back to their original positions
        priorityChanges?.filter((changeId) => changeId !== id);
        setPriorityChanges(priorityChanges);
      }
    }
  };

  const assignImageState = (descriptions: Description[]): Description[] => {
    for (let i = 0; i < descriptions.length; i += 1) {
      if (descriptions[i].text) {
        descriptions[i].is_text_not_image = true;
      } else {
        descriptions[i].is_text_not_image = false;
      }
    }
    return descriptions;
  };

  const assignProductInfo = () => {
    if (!productInfo) return;
    productName.current!.value = productInfo.display_name;
    productPrice.current!.value = productInfo.price_per_user.toString();
    productDescription.current!.value = productInfo.short_description;
  };

  /**
   * Sorts the descriptions in order by their priority property. Then
   * takes each pair of descriptions and creates a section from them.
   *
   * @param descriptions
   */
  const initializeSections = (descriptions: Description[]) => {
    descriptions.sort((a, b) => a.priority - b.priority);
    let sections: AccordionSectionProps[] = [];
    let k = 0;
    for (let i = 0; i < descriptions.length; i += 2) {
      k += 1;
      let newSection: AccordionSectionProps = {
        header: { title: "section " + k },
        rows: [],
        sectionID: k,
      };
      for (let j = 0; j < 2; j += 1) {
        if (descriptions[i + j]) {
          newSection.rows.push({
            component_id: descriptions[i + j].component_id,
            text: descriptions[i + j].is_text_not_image
              ? descriptions[i + j].text
              : undefined,
            image: descriptions[i + j].is_text_not_image
              ? undefined
              : descriptions[i + j].image,
            is_text_not_image: descriptions[i + j].is_text_not_image,
          });
        }
      }
      sections.push(newSection);
    }
    setSections(sections);
  };

  return (
    <>
      <HeaderEditPopup></HeaderEditPopup>
      <RowEditPopup></RowEditPopup>
      <TestimonialPopup product_id={productId!}></TestimonialPopup>
      <section className="container">
        <h2> Manage product</h2>
        <form className="wide-section">
          <label className="no-inline-margin" htmlFor="product-name">
            <b>Product name:</b>
          </label>
          <input
            type="text"
            id="product-name"
            name="product-name"
            ref={productName}
          />
          <label htmlFor="product-price">
            <b>Product price &#40;NOK&#41;:</b>
          </label>
          <input
            type="number"
            id="product-price"
            name="product-price"
            ref={productPrice}
          />
          <label htmlFor="product-image">
            <b>Upload header image</b>
          </label>
          <input
            type="file"
            id="product-image"
            name="product-image"
            accept="image/png, image/jpeg, image/webp"
            ref={productImage}
          />
          <p>Image: {productInfo?.main_image}</p>
          <label htmlFor="product-description">
            <b>Description:</b>
          </label>
          <textarea
            id="product-description"
            name="product-description"
            style={{
              fontSize: "1rem",
              fontFamily: "var(--ff-primary)",
              padding: "0.5em",
            }}
            rows={10}
            cols={50}
            ref={productDescription}
          />
        </form>
      </section>
      <section className="accordion-wrapper container">
        <AccordionTable
          sections={sections}
          testimonials={testimonials}
          productID={productId!}
          setTestimonials={setTestimonials}
          setSections={setSections}
        ></AccordionTable>
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
