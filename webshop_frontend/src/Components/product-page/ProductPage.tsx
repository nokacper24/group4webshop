import { useEffect, useState } from "react";
import { Link, useParams } from "react-router-dom";
import { Description, Product, Testimonial } from "../../Interfaces";
import DescriptionsContainer from "./DescriptionsContainer";
import Gallery from "./gallery/Gallery";
import PurchaseLicenseButton from "./PurchaseLicenseButton";
import Spinner from "../utils/utils";
import {
  fetchDescriptionComponents,
  fetchProduct,
  FetchError,
  fetchTestimonials,
} from "../../ApiController";
import { ErrorMessage } from "../ErrorMessage";

/**
 * The product page component.
 *
 * @returns the product page component
 */
export default function ProductPage() {
  const { productId } = useParams();

  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<Error | null>(null);

  const [product, setProduct] = useState<Product>();
  const [descriptions, setDescriptions] = useState<Description[]>([]);
  const [testimonials, setTestimonials] = useState<Testimonial[]>([]);

  useEffect(() => {
    fetchProduct(productId!)
      .then((product: Product) => {
        setProduct(product);
        setLoading(false);

        fetchTestimonials(product.product_id).then(
          (testimonials: Testimonial[]) => setTestimonials(testimonials)
        );
        fetchDescriptionComponents(product.product_id).then(
          (descriptions: Description[]) => setDescriptions(descriptions)
        );
      })
      .catch((error: unknown) => {
        if (error instanceof FetchError) {
          if (error.status === 404) {
            setError(
              new Error(`We could not find the product you are looking for.`)
            );
          } else {
            setError(
              new Error(`We are sorry: ${error.statusText}, ${error.message}`)
            );
          }
        } else {
          setError(new Error(`We are sorry: ${error}`));
        }
        setLoading(false);
      });
  }, []);

  return (
    <>
      {loading && <Spinner />}
      {error && (
        <>
          <ErrorMessage message={error.message} />
          <Link to="/products">
            <button className="banner-element hero-button">
              Back to products
            </button>
          </Link>
        </>
      )}
      {product && (
        <>
          <section
            className="banner"
            style={{ backgroundImage: `url(${product.main_image})` }}
          >
            <div className="banner-highlight">
              <h1 className="banner-title">{product.display_name}</h1>
              <p className="banner-description">{product.short_description}</p>
              <PurchaseLicenseButton active={product.available} />
              {product.available === false && <UnavailableTag />}
            </div>
          </section>
          {descriptions.length != 0 && (
            <section style={{ marginTop: "2em" }} className="container">
              {DescriptionsContainer(descriptions)}
            </section>
          )}
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
          <section className="highlight-section">
            <div className="container">
              <h2>Purchase</h2>
              <p>
                Purchase licenses for this product for your enterprise today!
              </p>
              <PurchaseLicenseButton active={product.available} />
            </div>
          </section>
        </>
      )}
    </>
  );
}

function UnavailableTag() {
  return (
    <div className="unavailable-tag">
      <p>Product is currently unavailable</p>
    </div>
  );
}
