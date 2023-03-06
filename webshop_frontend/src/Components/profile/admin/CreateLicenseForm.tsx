import { useEffect, useState, useRef } from "react";

type CompanyProps = {
  companyId: number;
  companyName: string;
};

type ProductProps = {
  productId: string;
  productName: string;
};

export default function CreateLicenseForm() {
  let baseUrl = import.meta.env.VITE_URL + ":" + import.meta.env.VITE_PORT;
  // Check if we are in production mode
  if (import.meta.env.PROD) {
    baseUrl = "";
  }

  const [companies, setCompanies] = useState<CompanyProps[]>([]);
  const [products, setProducts] = useState<ProductProps[]>([]);

  const fetchCompanies = async () => {
    const response = await fetch(`${baseUrl}/api/companies`);
    const data = await response.json();
    const companies: CompanyProps[] = data.map((company: any) => {
      return {
        companyId: company.company_id,
        companyName: company.company_name,
      };
    });
    return companies;
  };

  const fetchProducts = async () => {
    const response = await fetch(`${baseUrl}/api/products`);
    const data = await response.json();
    const products: ProductProps[] = data.map((product: any) => {
      return {
        productId: product.product_id,
        productName: product.display_name,
      };
    });
    return products;
  };

  useEffect(() => {
    fetchCompanies()
      .then((companies) => {
        setCompanies(companies);
      })
      .catch(() => console.error("Failed to load companies"));

    fetchProducts()
      .then((products) => {
        setProducts(products);
      })
      .catch(() => console.error("Failed to load products"));
  }, []);

  const postLicense = async (license: any) => {
    fetch(`${baseUrl}/api/licenses`, {
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
        location.reload();
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
      console.log("License values are invalid");
    }
  };

  return (
    <form
      className="form-container container"
      onSubmit={(event) => handleSubmit(event)}
    >
      {/* Prevent implicit submission of the form */}
      <button
        type="submit"
        disabled
        style={{ display: "none" }}
        aria-hidden="true"
      ></button>
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
            <option key={company.companyId} value={company.companyId}>
              {company.companyName}
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
            <option key={product.productId} value={product.productId}>
              {product.productName}
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
        Save
      </button>
    </form>
  );
}
