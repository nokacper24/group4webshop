import { useRef } from "react";
import { useEffect, useState } from "react";
import { MeUser, Product } from "../../Interfaces";
import ProductSelect from "./ProductSelect";
import {
  fetchMe,
  fetchAvailableProducts,
  sendSupportTicket,
} from "../../ApiController";
import { Link } from "react-router-dom";

/**
 * Represents a Support Form component.
 * Contains product select, subject and message of support ticket.
 *
 * @returns A Support Form component.
 */
export default function SupportForm() {
  const [user, setUser] = useState<MeUser>();
  const [products, setProducts] = useState<Product[]>([]);

  const productSelect = useRef<HTMLSelectElement>(null);
  const [formAlert, setFormAlert] = useState<string>("");
  let userEmail;

  useEffect(() => {
    fetchAvailableProducts().then((products: Product[]) =>
      setProducts(products)
    );
    fetchMe()
      .then((user: MeUser) => {
        setUser(user);
      })
      .catch((error) => {});

    if (user && user.email) {
      userEmail = (
        <p>
          You are signed in as:
          <br />
          <span className="user-email">{user.email}</span>
        </p>
      );
    } else {
      userEmail = (
        <p>
          You are not signed in. Please <Link to="/profile">sign in</Link> to
          use the form.
        </p>
      );
    }
  }, []);

  /**
   * Handle the submit of the support form. Validates the form data
   * and sends the support ticket to ProFlex.
   *
   * @param event The form event.
   */
  const handleSubmit = async (event: React.FormEvent<HTMLFormElement>) => {
    event.preventDefault();

    if (productSelect.current && productSelect.current.selectedIndex == 0) {
      setFormAlert("Please select a product");
    } else {
      setFormAlert("");
    }

    let product = productSelect.current?.value;

    if (product !== undefined) {
      const result = await sendSupportTicket(
        product,
        event.currentTarget["support-subject"].value,
        event.currentTarget["support-message"].value
      );

      if (result) {
        setFormAlert("Your message has been sent");
      } else {
        setFormAlert("There was an error sending your message");
      }
    } else {
      setFormAlert("Please select a product");
    }
  };

  return (
    <form
      className="container form-container"
      onSubmit={(event) => handleSubmit(event)}
    >
      <h2>Contact support</h2>
      {userEmail}

      <ProductSelect ref={productSelect} products={products} />

      <label htmlFor="support-subject">Subject</label>
      <input
        id="support-subject"
        name="support-subject"
        type="text"
        placeholder="Subject"
        required
      ></input>

      <label htmlFor="support-message">Message</label>
      <textarea
        id="support-message"
        name="support-message"
        placeholder="Describe the issue"
        required
      ></textarea>

      <button
        className="default-button submit-button m-t-1"
        type="submit"
        disabled={user ? false : true}
      >
        Send
      </button>
      <p className="form-alert">{formAlert}</p>
    </form>
  );
}
