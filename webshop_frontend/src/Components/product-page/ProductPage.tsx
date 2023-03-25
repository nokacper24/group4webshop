import { useEffect, useState } from "react";
import { useParams } from "react-router-dom";
import { Description, Product, Testimonial } from "../../Interfaces";
import DescriptionsContainer from "./DescriptionsContainer";
import Gallery from "./gallery/Gallery";
import PurchaseLicenseButton from "./PurchaseLicenseButton";

let counter = 0;

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
  const { productId } = useParams();

  const [product, setProduct] = useState<Product>();
  const [descriptions, setDescriptions] = useState<Description[]>([]);
  const [testimonials, setTestimonials] = useState<Testimonial[]>([]);

  const fetchProduct = async () => {
    const response = await fetch(`${baseUrl}/api/products/${productId}`);
    const data: Product = await response.json();
    setProduct(data);
  };

  const fetchDescriptionComponents = async () => {
    const response = await fetch(
      `${baseUrl}/api/products/${productId}/descriptions`
    );
    const data: Description[] = await response.json();
    setDescriptions(data);
  };

  const fetchTestimonials = async () => {
    const response = await fetch(`${baseUrl}/api/testimonials/${productId}`);
    const data: Testimonial[] = await response.json();
    setTestimonials(data);
  };

  useEffect(() => {
    fetchProduct();
    fetchDescriptionComponents();
    fetchTestimonials();
  }, []);

  return (
    <>
      {product && (
        <>
          <section className="banner">
            <div className="banner-inner">
              <div className="banner-highlight">
                <h1 className="banner-title">{product.display_name}</h1>
                <PurchaseLicenseButton />
              </div>
            </div>
          </section>
          <hr></hr>
          <section className="product-description-container container">
            {DescriptionsContainer(descriptions)}
          </section>
          {testimonials.length > 0 && (
            <section className="gallery-wrapper">
              <div className="container">
                <h2 className="testimonial-title">Testimonials</h2>
                <Gallery
                  testimonials={testimonials}
                  galleryName={"Testimonials"}
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
