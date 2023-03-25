import { useEffect, useRef, useState } from "react";
import { useParams } from "react-router-dom";
import { Product } from "../../Interfaces";
import LicensePrices from "./LicensePrices";

/**
 * Represents a Purchase License page.
 * Contains a short product description, the plan options, and
 * a checkbox for accepting terms and conditions.
 *
 * @returns The Purchase License page component.
 */
export default function PurchaseLicense() {
  let baseUrl = import.meta.env.VITE_URL + ":" + import.meta.env.VITE_PORT;
  // Check if we are in production mode
  if (import.meta.env.PROD) {
    baseUrl = "";
  }

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

  const fetchProduct = async () => {
    const response = await fetch(`${baseUrl}/api/products/${productId}`);
    const data = await response.json();
    const product: Product = data;
    setProduct(product);
  };

  /**
   * Update the total price in the object's state.
   *
   * @param event The user event.
   */
  const updatePrice = (event: React.ChangeEvent<HTMLSelectElement>) => {
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
   * Send a POST request to create a license.
   *
   * @param license The license to create.
   */
  const postLicense = async (license: any) => {
    fetch(`${baseUrl}/api/priv/licenses`, {
      method: "POST",
      headers: {
        Accept: "application/json",
        "Content-Type": "application/json",
      },
      body: license,
    }).then((response) => {
      const status = response.status;
      if (status == 201) {
        alert("License successfully purchased!");
        location.reload();
      } else {
        alert(
          "Sorry, something went wrong when purchasing the license. Try again."
        );
      }
    });
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
      let license = JSON.stringify({
        company_id: 1 /* TODO: Get real company */,
        product_id: productId,
        start_date: new Date(),
        end_date: new Date(
          new Date().setFullYear(new Date().getFullYear() + 1)
        ),
        amount: Math.round(totalPrice / product.price_per_user),
        valid: true,
      });

      postLicense(license);
    }
  };

  useEffect(() => {
    fetchProduct();
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

          <button type="submit" className="default-button submit-button">
            Buy
          </button>
          <div className="checkbox-input">
            <input id="accept-terms" type="checkbox" required />
            <label htmlFor="accept-terms">
              I have read and agree to the <a href="#!">terms of service</a>.
            </label>
          </div>
          <p className="form-alert" ref={formAlert}></p>
        </form>
      </section>
    </>
  );
}
