import { ChangeEvent, useEffect, useRef, useState } from "react";
import { useNavigate, useParams } from "react-router-dom";
import { License, Product } from "../../Interfaces";
import LicensePrices from "./LicensePrices";
import { fetchProduct, postLicense } from "../../ApiController";
import TermsOfService from "../profile/register/TermsOfService";

/**
 * Represents a Purchase License page.
 * Contains a short product description, the plan options, and
 * a checkbox for accepting terms and conditions.
 *
 * @returns The Purchase License page component.
 */
export default function PurchaseLicense() {
  const navigate = useNavigate();

  const { productId } = useParams();
  const [product, setProduct] = useState<Product>({
    product_id: "",
    display_name: "",
    price_per_user: 0,
    short_description: "",
    main_image: "",
    available: false,
  });
  const [totalPrice, setTotalPrice] = useState<number>(0);

  const price = useRef<HTMLSelectElement>(null);
  const formAlert = useRef<HTMLParagraphElement>(null);

  /**
   * Update the total price in the object's state.
   *
   * @param event The user event.
   */
  const updatePrice = (event: ChangeEvent<HTMLSelectElement>) => {
    setTotalPrice(parseInt(event.target.value));
  };

  /**
   * Validate that the form has valid values.
   *
   * @returns true if the form is valid, false if the form is not valid.
   */
  const validateForm = () => {
    if (price.current && formAlert.current) {
      if (parseInt(price.current.value) > 0) {
        formAlert.current.innerHTML = "";
        return true;
      } else {
        formAlert.current.innerHTML = "Please select a plan";
      }
    } else {
      return false;
    }
  };

  /**
   * Check if the form has valid values, and purchase licenses
   * with selected plans if form is valid.
   *
   * @param event The form submit event.
   */
  const handleSubmit = (event: React.FormEvent<HTMLFormElement>) => {
    event.preventDefault();

    if (validateForm()) {
      let license: License = {
        license_id: NaN,
        company_id: NaN /* TODO: Get real company */,
        product_id: product.product_id,
        product_name: product.display_name,
        start_date: new Date(),
        end_date: new Date(
          new Date().setFullYear(new Date().getFullYear() + 1)
        ),
        amount: Math.round(totalPrice / product.price_per_user),
        valid: true,
      };

      postLicense(license).then((response: Response) => {
        if (response.status == 201) {
          alert("License successfully purchased");
          // Refresh
          navigate(0);
        } else {
          alert(
            "Sorry, something went wrong when purchasing license. Please try again."
          );
        }
      });
    }
  };

  useEffect(() => {
    fetchProduct(productId!).then((product: Product) => setProduct(product));
  }, []);

  return (
    <>
      <section className="container">
        <h1>Purchase License</h1>
        <form
          className="left-aligned"
          onSubmit={(event) => handleSubmit(event)}
        >
          <h2>{product.display_name}</h2>
          <p>
            {product.short_description}
            <br></br>
            Purchase a license for your company. Licenses are valid for a year,
            and will automatically be renewed unless you cancel it.
          </p>
          <LicensePrices
            price={product.price_per_user}
            updatePrice={(event) => updatePrice(event)}
            refs={{ price }}
          />
          <p className="total-price">TOTAL: ${totalPrice}</p>

          <TermsOfService />
          <p className="form-alert" ref={formAlert}></p>

          <button type="submit" className="default-button submit-button">
            Buy
          </button>
        </form>
      </section>
    </>
  );
}
