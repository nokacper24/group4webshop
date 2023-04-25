import { useEffect, useState, useRef } from "react";
import { Company, Product } from "../../../Interfaces";
import { useNavigate } from "react-router-dom";
import { fetchCompanies, fetchProducts } from "../../../ApiController";

/**
 * A form for creating a license.
 *
 * @returns A create license form component.
 */
export default function CreateLicenseForm() {
  let baseUrl = import.meta.env.VITE_URL + ":" + import.meta.env.VITE_PORT;
  // Check if we are in production mode
  if (import.meta.env.PROD) {
    baseUrl = "";
  }

  const navigate = useNavigate();

  const [companies, setCompanies] = useState<Company[]>([]);
  const [products, setProducts] = useState<Product[]>([]);

  useEffect(() => {
    fetchCompanies()
      .then((companies: Company[]) => {
        setCompanies(companies);
      })
      .catch(() => alert("Failed to load companies"));

    fetchProducts()
      .then((products: Product[]) => {
        setProducts(products);
      })
      .catch(() => alert("Failed to load products"));
  }, []);

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
        alert("License created.");
        // Refresh
        navigate(0);
      } else {
        alert("Something went wrong when creating the license. Try again.");
      }
    });
  };

  const company = useRef<HTMLSelectElement>(null);
  const product = useRef<HTMLSelectElement>(null);
  const start = useRef<HTMLInputElement>(null);
  const end = useRef<HTMLInputElement>(null);
  const amount = useRef<HTMLInputElement>(null);

  /**
   * When user submits the form, take in inputs and create license.
   *
   * @param event The form submit event.
   */
  const handleSubmit = (event: React.FormEvent<HTMLFormElement>) => {
    event.preventDefault();

    if (
      company.current &&
      product.current &&
      start.current &&
      end.current &&
      amount.current
    ) {
      let license = JSON.stringify({
        company_id: parseInt(company.current.value),
        product_id: product.current.value,
        start_date: new Date(start.current.value),
        end_date: new Date(end.current.value),
        amount: parseInt(amount.current.value),
        valid: true,
      });

      postLicense(license);
    } else {
      alert("License values are invalid");
    }
  };

  return (
    <form
      className="form-container container"
      onSubmit={(event) => handleSubmit(event)}
    >
      <h2>Create license</h2>

      <label htmlFor="companies">Company</label>
      <select
        ref={company}
        id="companies"
        name="company_id"
        defaultValue="-1"
        required
      >
        <option key="-1" value="-1" disabled hidden>
          Please choose a company
        </option>
        {companies.map((company) => {
          return (
            <option key={company.company_id} value={company.company_id}>
              {company.company_name}
            </option>
          );
        })}
      </select>

      <label htmlFor="products">Product</label>
      <select
        ref={product}
        id="products"
        name="product_id"
        defaultValue="-1"
        required
      >
        <option key="-1" value="-1" disabled hidden>
          Please choose a product
        </option>
        {products.map((product) => {
          return (
            <option key={product.product_id} value={product.product_id}>
              {product.display_name}
            </option>
          );
        })}
      </select>

      <label htmlFor="start-date">Start date</label>
      <input
        ref={start}
        id="start-date"
        name="start_date"
        type="date"
        required
      ></input>

      <label htmlFor="end-date">End date</label>
      <input
        ref={end}
        id="end-date"
        name="end_date"
        type="date"
        required
      ></input>

      <label htmlFor="amount">Amount (max users)</label>
      <input
        ref={amount}
        id="amount"
        name="amount"
        type="number"
        placeholder="Example: 10"
        required
      ></input>

      <button type="submit" className="m-t-1 default-button">
        Save changes
      </button>
    </form>
  );
}
