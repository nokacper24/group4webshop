import { MouseEvent as ReactMouseEvent } from "react";
import { useEffect, useState } from "react";
import { MeUser, Product } from "../../Interfaces";
import ProductSelect from "./ProductSelect";
import { fetchMe, fetchProducts } from "../../ApiController";
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

  useEffect(() => {
    fetchProducts().then((products: Product[]) => setProducts(products));
    fetchMe().then((user: MeUser) => {
      setUser(user);
    });
  }, []);

  let userEmail;
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
        You are not signed in. Please <Link to="/profile">sign in</Link> to use
        the form.
      </p>
    );
  }

  return (
    <form className="container form-container">
      <h2>Contact support</h2>
      {userEmail}

      <ProductSelect products={products} />

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
        onClick={(event) => validateForm(event)}
        disabled={user ? false : true}
      >
        Send
      </button>
      <p className="form-alert"></p>
    </form>
  );
}

/**
 * Confirm that all the form's input is valid.
 *
 * If the user has not selected an option for the product,
 * inform the user that their input is needed.
 *
 * @param event Mouse Event on button
 */
function validateForm(
  event: ReactMouseEvent<HTMLButtonElement, MouseEvent>
): void {
  const productSelect: HTMLSelectElement | null =
    document.querySelector("#product-select");
  const formAlert: HTMLElement | null = document.querySelector(".form-alert");

  if (productSelect?.selectedIndex == 0) {
    event.preventDefault();

    if (formAlert != null) {
      formAlert.innerHTML = "Please select a product";
    }
  }
}
