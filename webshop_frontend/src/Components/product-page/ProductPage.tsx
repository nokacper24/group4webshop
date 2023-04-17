import { useEffect, useState } from "react";
import { useParams } from "react-router-dom";
import { Description, Product, Testimonial } from "../../Interfaces";
import DescriptionsContainer from "./DescriptionsContainer";
import Gallery from "./gallery/Gallery";
import PurchaseLicenseButton from "./PurchaseLicenseButton";
import {
  fetchDescriptionComponents,
  fetchProduct,
  fetchTestimonials,
} from "../../ApiController";

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

  useEffect(() => {
    fetchProduct(productId!).then((product: Product) => {
      if (product.product_id) {
        setProduct(product);
        fetchTestimonials(product.product_id).then(
          (testimonials: Testimonial[]) => setTestimonials(testimonials)
        );
        fetchDescriptionComponents(product.product_id).then(
          (descriptions: Description[]) => setDescriptions(descriptions)
        );
      }
    });
  }, []);

  return (
    <>
      {product && (
        <>
          <section className="banner" style={{backgroundImage:"url(/"+product.main_image+")"}}>
            <div className="banner-inner">
              <div className="banner-highlight">
                <h1 className="banner-title">{product.display_name}</h1>
                <p className="banner-description">{product.short_description}</p>
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
        <section className="container">
          <h1>Product not found</h1>
          <p>Sorry, could not find the product you were looking for!</p>
        </section>
      )}
    </>
  );
}
