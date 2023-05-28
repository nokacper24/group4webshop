import { useParams } from "react-router-dom";
import AccordionTable from "./Accordion/AccordionTable";
import HeaderEditPopup from "./Edit-popups/HeaderEditPopup";
import RowEditPopup from "./Edit-popups/RowEditPopup";
import { useEffect, useRef, useState } from "react";
import TestimonialPopup from "./Edit-popups/TestimonialPopup";
import {
  Description,
  Product,
  LocalDescription,
  Testimonial,
} from "../../../../Interfaces";
import {
  fetchDescriptionComponents,
  fetchProduct,
  fetchTestimonials,
} from "../../../../ApiController";
import { AccordionSectionProps } from "./Accordion/AccordionSection";
import { ChangeType } from "./Accordion/ChangeTypes";

let baseUrl = import.meta.env.VITE_URL + ":" + import.meta.env.VITE_PORT;
// Check if we are in production mode
if (import.meta.env.PROD) {
  baseUrl = "";
}

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
  const productForm = useRef<HTMLFormElement>(null);

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

  const [priorityChanges, setPriorityChanges] = useState<number[][]>([]); //Section IDs that has had a swap change as value
  const [contentChanges, setContentChanges] = useState<
    Map<ChangeType, number[]>
  >(new Map()); //The type of change as key, list of IDs that has had that change as value
  const [testimonialChanges, setTestimonialChanges] = useState<
    Map<ChangeType, number[]>
  >(new Map()); //The type of change as key, list of IDs that has had that change as value

  useEffect(() => {
    // Sets up the map so that it can register changes
    initializeContentAndTestimonialChanges();
  }, []);

  function initializeContentAndTestimonialChanges() {
    setContentChanges((changes) => {
      for (let type in ChangeType) {
        changes.set(ChangeType[type as keyof typeof ChangeType], []);
      }
      return changes;
    });
    setTestimonialChanges((changes) => {
      for (let type in ChangeType) {
        changes.set(ChangeType[type as keyof typeof ChangeType], []);
      }
      return changes;
    });
  }

  /**
   * Registers changes to the content of the table. This is used to keep track of what has changed and what needs to be saved.
   *
   * @param id The ID of the section that has changed
   * @param change The type of change that has been made
   */
  const registerContentChange = (id: number, change: ChangeType) => {
    if (contentChanges.get(change)) {
      console.log(contentChanges.get(change));
      console.log(contentChanges);
    }
    if (!contentChanges.get(change)?.includes(id)) {
      contentChanges.get(change)?.push(id);
    }
    if (change === ChangeType.Delete) {
      contentChanges
        .get(ChangeType.Edit)
        ?.filter((changeId) => changeId !== id);
      priorityChanges?.filter((array) => array[0] !== id && array[1] !== id);
      setPriorityChanges(priorityChanges);
    } else if (change === ChangeType.Swap) {
      if (
        !priorityChanges?.find((array) => array[0] === id || array[1] === id)
      ) {
        let rows: number[] = [];
        sections
          .find((sections) => sections.sectionID === id)
          ?.rows.forEach((row) => rows.push(row.component_id));
        priorityChanges?.push(rows);
        setPriorityChanges(priorityChanges);
      } else {
        //Since a section only can have two rows at a time, we can assume that if the section already is on the list, the rows are swapped back to their original positions
        priorityChanges?.filter((array) => array[0] !== id && array[1] !== id);
        setPriorityChanges(priorityChanges);
      }
    }
    setContentChanges(contentChanges);
  };

  const registerTestimonialChange = (id: number, change: ChangeType) => {
    if (!testimonialChanges.get(change)?.includes(id)) {
      testimonialChanges.get(change)?.push(id);
    }
    if (change === ChangeType.Delete) {
      testimonialChanges
        .get(ChangeType.Edit)
        ?.filter((changeId) => changeId !== id);
    }
    setTestimonialChanges(testimonialChanges);
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

  const [hidden, setHidden] = useState<boolean>(false);

  const initializeAvailabilityChangeProtocol = () => {
    let confirmChange = confirm(
      hidden
        ? "Do you want to unhide this product? \n This will make this product visible on the webshop."
        : "Do you want to hide this product? \n This will make this product hidden from the webshop."
    );
    if (confirmChange) {
      setHidden(!hidden);
    }
  };

  /**
   * Starts the process of saving the product. This includes saving the product itself, the sections and the testimonials.
   */
  const initializeSaveProtocol = (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    sendProduct();
    sendDeleteDescriptions();
    sendNewDescriptions();
    sendPriorityChanges();
    sendEdits();

    sendDeletedTestimonials();
    sendNewTestimonials();
    sendEditedTestimonials();

    alert("Product saved!");
  };

  const sendProduct = async () => {
    let formData = new FormData();
    formData.append("image", productImage.current!.files![0]);
    formData.append("price_per_unit", productPrice.current!.value);
    formData.append("product_name", productName.current!.value);
    formData.append("short_description", productDescription.current!.value);
    let response = await fetch(
      `${baseUrl}/api/priv/products${createState ? "" : `/${productId}`}`,
      {
        method: createState ? "POST" : "PUT",
        headers: {
          Accept: "multipart/form-data",
        },
        body: formData,
      }
    );
  };

  const sendDeleteDescriptions = async () => {
    contentChanges?.get(ChangeType.Delete)?.forEach((id) => {
      let response = fetch(
        `${baseUrl}/api/priv/products/${productId}/descriptions/${id}`,
        {
          method: "DELETE",
          headers: {
            "Content-Type": "application/json",
          },
        }
      );
    });
  };

  const findRow = (
    id: number
  ): { description: LocalDescription | undefined; foundAt: number } => {
    for (let i = 0; i < sections.length; i += 1) {
      for (let j = 0; j < sections[i].rows.length; j += 1) {
        if (sections[i].rows[j].component_id === id) {
          return { description: sections[i].rows[j], foundAt: i + j + 2 };
        }
      }
    }
    return { description: undefined, foundAt: -1 };
  };

  const sendNewDescriptions = async () => {
    let found = false;
    console.log(contentChanges);
    contentChanges?.get(ChangeType.Add)?.forEach((id) => {
      console.log(
        "Gotten contentChange " + contentChanges?.get(ChangeType.Add)
      );
      let { description: row, foundAt } = findRow(id);
      if (row) {
        if (row.is_text_not_image) {
          let respone = fetch(
            `${baseUrl}/api/priv/products/${productId}/descriptions/text`,
            {
              method: "POST",
              headers: {
                "Content-Type": "application/json",
              },
              body: JSON.stringify({
                text_title: row.text?.text_title,
                paragraph: row.text?.paragraph,
              }),
            }
          );
        } else {
          const formData = new FormData();
          let image_content: Blob;
          if (row.image instanceof File) {
            image_content = row.image;
          }
          formData.append("image", image_content!);
          formData.append("alt_text", row.image?.alt_text!);
          let response = fetch(
            `${baseUrl}/api/priv/products/${productId}/descriptions/image`,
            {
              method: "POST",
              headers: {
                "Content-Type": "multipart/form-data",
                Accept: "multipart/form-data",
              },
              body: formData,
            }
          );
        }
      }
    });
  };

  const sendPriorityChanges = async () => {
    priorityChanges?.forEach((rows) => {
      let response = fetch(
        `${baseUrl}/api/priv/products/${productId}/descriptions/priorityswap`,
        {
          method: "PATCH",
          headers: {
            "Content-Type": "application/json",
          },
          body: JSON.stringify([rows[0], rows[1]]),
        }
      );
    });
  };

  const sendEdits = async () => {
    contentChanges?.get(ChangeType.Edit)?.forEach((id) => {
      let { description: row, foundAt } = findRow(id);
      if (row) {
        if (row.is_text_not_image) {
          let respone = fetch(
            `${baseUrl}/api/priv/products/${productId}/descriptions/text/${id}`,
            {
              method: "PUT",
              headers: {
                "Content-Type": "application/json",
              },
              body: JSON.stringify({
                component_id: id,
                full_width: false,
                image: null,
                priority: foundAt,
                product_id: productId,
                text: row.text,
              }),
            }
          );
        } else {
          const formData = new FormData();
          let image_content: Blob;
          if (row.image instanceof File) {
            image_content = row.image;
          }
          formData.append("image", image_content!);
          formData.append("alt_text", row.image?.alt_text!);
          let response = fetch(
            `${baseUrl}/api/priv/products/${productId}/descriptions/image/${id}`,
            {
              method: "PUT",
              headers: {
                "Content-Type": "multipart/form-data",
                Accept: "multipart/form-data",
              },
              body: formData,
            }
          );
        }
      }
    });
  };

  const sendDeletedTestimonials = async () => {
    testimonialChanges?.get(ChangeType.Delete)?.forEach((id) => {
      let response = fetch(
        `${baseUrl}/api/priv/testimonials/${productId}/${id}`,
        {
          method: "DELETE",
          headers: {
            "Content-Type": "application/json",
          },
        }
      );
    });
  };

  const sendNewTestimonials = async () => {
    let formData = new FormData();
    testimonialChanges?.get(ChangeType.Add)?.forEach((id) => {
      let testimonial = testimonials.find((t) => {
        return t.testimonial_id === id;
      });
      if (testimonial) {
        formData.append("author", testimonial.author);
        formData.append(
          "author_pic",
          typeof testimonial.author_pic === "string"
            ? ""
            : (testimonial.author_pic as Blob)
        );
        formData.append("product_id", productId!);
        formData.append("text", testimonial.text);
        formData.append(
          "testimonial_id",
          testimonial.testimonial_id.toString()
        );
        let response = fetch(`${baseUrl}/api/priv/testimonials/${productId}`, {
          method: "POST",
          headers: {
            "Content-Type": "multipart/form-data",
            Accept: "multipart/form-data",
          },
          body: JSON.stringify({
            formData,
          }),
        });
      }
    });
  };

  const sendEditedTestimonials = async () => {
    testimonialChanges?.get(ChangeType.Edit)?.forEach((id) => {
      let testimonial = testimonials.find((t) => {
        return t.testimonial_id === id;
      });
      if (testimonial) {
        let formData = new FormData();
        formData.append("author", testimonial.author);
        formData.append(
          "author_pic",
          typeof testimonial.author_pic === "string"
            ? ""
            : (testimonial.author_pic as Blob)
        );
        formData.append("product_id", productId!);
        formData.append("text", testimonial.text);
        formData.append(
          "testimonial_id",
          testimonial.testimonial_id.toString()
        );
        let response = fetch(
          `${baseUrl}/api/priv/testimonials/${productId}/${id}`,
          {
            method: "PUT",
            headers: {
              "Content-Type": "multipart/form-data",
              Accept: "multipart/form-data",
            },
            body: JSON.stringify({
              formData,
            }),
          }
        );
      }
    });
  };

  return (
    <>
      <HeaderEditPopup></HeaderEditPopup>
      <RowEditPopup></RowEditPopup>
      <TestimonialPopup product_id={productId!}></TestimonialPopup>
      <form
        className="wide-section"
        action={`${baseUrl}/api/priv/products${
          createState ? "" : `/${productId}`
        }`}
        method={createState ? "POST" : "PUT"}
        encType="multipart/form-data"
        onSubmit={initializeSaveProtocol}
        ref={productForm}
      >
        <section className="container">
          <h2> Manage product</h2>

          <label className="no-inline-margin" htmlFor="product-name">
            <b>Product name:</b>
          </label>
          <input
            type="text"
            id="product-name"
            name="product_name"
            maxLength={100}
            ref={productName}
            required={true}
          />
          <label htmlFor="product-price">
            <b>Product price &#40;NOK&#41;:</b>
          </label>
          <input
            type="number"
            step={0.01}
            id="product-price"
            name="price_per_unit"
            ref={productPrice}
            required={true}
          />
          <label htmlFor="product-image">
            <b>Upload header image</b>
          </label>
          <input
            type="file"
            id="product-image"
            name="image"
            accept="image/png, image/jpeg, image/webp"
            ref={productImage}
            required={true}
          />
          <p>Image: {productInfo?.main_image}</p>
          <label htmlFor="product-description">
            <b>Description:</b>
          </label>
          <textarea
            id="product-description"
            name="short_description"
            style={{
              fontSize: "1rem",
              fontFamily: "var(--ff-primary)",
              padding: "0.5em",
            }}
            rows={10}
            cols={50}
            maxLength={255}
            ref={productDescription}
            required={true}
          />
        </section>
        <section className="accordion-wrapper container">
          <AccordionTable
            sections={sections}
            testimonials={testimonials}
            productID={productId!}
            registerContentChange={registerContentChange}
            registerTestimonialChange={registerTestimonialChange}
            setTestimonials={setTestimonials}
            setSections={setSections}
          ></AccordionTable>
        </section>
        <section className="button-container">
          <button
            className="default-button small-button bg-danger"
            onClick={() => initializeAvailabilityChangeProtocol()}
            type="button"
          >
            {hidden ? "Unhide" : "Hide"}
          </button>
          <button className="default-button small-button" type="submit">
            Save
          </button>
        </section>
      </form>
    </>
  );
}
