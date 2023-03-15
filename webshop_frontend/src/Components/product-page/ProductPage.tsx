import { useEffect, useState } from "react";
import { useParams } from "react-router-dom";
import { Description, Product } from "../../Interfaces";
import DescriptionRow, { ProductPageRow } from "./DescriptionRow";
import Gallery from "./gallery/Gallery";
import { GalleryProps } from "./gallery/Gallery";
import { SlideType } from "./gallery/SlideTypes";
import PurchaseLicenseButton from "./PurchaseLicenseButton";

let counter = 0;

interface Testimonial {
  testimonial_id: number;
  author: string;
  text: string;
  author_pic: string;
}

let baseUrl = import.meta.env.VITE_URL + ":" + import.meta.env.VITE_PORT;
// check if we are in production mode
if (import.meta.env.PROD) {
  baseUrl = "";
}

/**
 * The product page component.
 *
 * @returns the product page component
 */
export default function ProductPage() {
  const [product, setProduct] = useState<Product>();
  // array of rows
  const [descriptionRow, setDescriptionRow] = useState<ProductPageRow[]>([]);

  //get product id from url
  const { productId } = useParams();

  const fetchProduct = async () => {
    const response = await fetch(`${baseUrl}/api/products/${productId}`);
    const data = await response.json();
    const product: Product = data;
    setProduct(product);
  };

  const fetchDescriptions = async () => {
    const response = await fetch(
      `${baseUrl}/api/products/${productId}/descriptions`
    );
    const data = await response.json();
    const descriptions: Description[] = data.map((description: Description) => {
      // check if text or image, by seeing if either text or image is null

      if (description.text == null) {
        return {
          component_id: description.component_id,
          priority: description.priority,
          product_id: description.product_id,
          text: {
            text_title: null,
            paragraph: null,
          },
          image: {
            image_path: description.image.image_path,
            alt_text: description.image.alt_text,
          },
          isTextNotImage: true,
        };
      } else {
        return {
          component_id: description.component_id,
          priority: description.priority,
          product_id: description.product_id,
          text: {
            text_title: description.text.text_title,
            paragraph: description.text.paragraph,
          },
          image: {
            image_path: null,
            alt_text: null,
          },
          isTextNotImage: false,
        };
      }
    });

    // sort by priority
    descriptions.sort(
      (a: { priority: number }, b: { priority: number }) =>
        a.priority - b.priority
    );

    // create rows
    let rows: ProductPageRow[] = [];

    descriptions.forEach((description: Description) => {
      // check if description is text or image
      if (description.isTextNotImage) {
        // create row with text
        let row: ProductPageRow = {
          props: {
            item1: {
              title: description.text.text_title,
              content: description.text.paragraph,
              isTextNotImage: true,
            },
            item2: {
              title: "",
              content: description.image.image_path,
              isTextNotImage: false,
            },
            textToLeft: true,
          },
        };
        rows.push(row);
      } else {
        // create row with image
        let row: ProductPageRow = {
          props: {
            item1: {
              title: "",
              content: description.image.image_path,
              isTextNotImage: false,
            },
            item2: {
              title: description.text.text_title,
              content: description.text.paragraph,
              isTextNotImage: true,
            },
            textToLeft: false,
          },
        };
        rows.push(row);
      }
    });

    setDescriptionRow(rows);
  };

  const [testimonials, setTestimonials] = useState<GalleryProps>({
    galleryName: "",
    slides: [
      {
        slideId: "",
        mainContent: "",
        reviewerProfile: {
          picturePath: "",
          name: "",
        },
        slideType: SlideType.PARAGRAPH,
      },
    ],
  });

  const fetchTestimonials = async () => {
    const response = await fetch(`${baseUrl}/api/testimonials/${productId}`);
    const data = await response.json();
    return data.map((testimonial: Testimonial) => testimonial);
  };

  useEffect(() => {
    fetchProduct();
    fetchDescriptions();
    fetchTestimonials().then((testimonials) => {
      setTestimonials({
        galleryName: "Testimonials",
        slides: testimonials.map((testimonial: any) => {
          return {
            slideId: testimonial.author,
            mainContent: testimonial.text,
            reviewerProfile: {
              picturePath: testimonial.author_pic,
              name: testimonial.author,
            },
            slideType: SlideType.PARAGRAPH,
          };
        }),
      });
    });
  }, []);

  return (
    <>
      {product && (
        <>
          <section className="banner">
            <div className="banner-inner">
              <div className="banner-highlight">
                <h1 className="banner-title banner-element hero-title">
                  {product.display_name}
                </h1>
                <PurchaseLicenseButton />
              </div>
            </div>
          </section>
          <hr></hr>
          <section className="product-description container">
            {descriptionRow.map((productPageRow) => (
              <DescriptionRow
                key={assignUniqueKey("row")}
                props={productPageRow.props}
              />
            ))}
          </section>
          {testimonials.slides.length > 0 && (
            <section className="gallery-wrapper">
              <div className="container">
                <h2 className="testimonial-title">Testimonials</h2>
                <Gallery
                  slides={testimonials.slides}
                  galleryName={testimonials.galleryName}
                />
              </div>
            </section>
          )}
          <section className="container">
            <h2>Purchase</h2>
            <p>Purchase licenses for this product for your enterprise today!</p>
            <PurchaseLicenseButton />
          </section>
        </>
      )}
      {!product && (
        <div className="container">
          <h1>Product not found</h1>
        </div>
      )}
    </>
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
  return base + counter;
}
