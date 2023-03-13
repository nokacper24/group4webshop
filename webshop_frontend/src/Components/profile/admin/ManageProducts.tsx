import { useEffect, useState } from "react";
import { SelectTableRowProps } from "../managing/ManageLicenseAccess";
import SelectTable from "../managing/SelectTable";

type ProductProps = {
  id: string;
  name: string;
  description: string;
};

export default function ManageProducts() {
  let baseUrl = import.meta.env.VITE_URL + ":" + import.meta.env.VITE_PORT;
  // Check if we are in production mode
  if (import.meta.env.PROD) {
    baseUrl = "";
  }

  const [products, setProducts] = useState<SelectTableRowProps[]>([]);

  const editProduct = (index: number) => {
    console.log("Editing: ", index);
  };

  const productsList = {
    header: {
      columns: [{ text: "Product" }, { text: "Description" }],
    },
    rows: products,
    button: { text: "Edit", action: editProduct },
    outsideButtons: [],
  };

  const fetchProducts = async () => {
    const response = await fetch(`${baseUrl}/api/products`);
    const data = await response.json();
    const products: ProductProps[] = data.map((product: any) => {
      return {
        id: product.product_id,
        name: product.display_name,
        description: product.short_description,
      };
    });
    return products;
  };

  useEffect(() => {
    fetchProducts().then((products) => {
      setProducts(
        products.map((product) => {
          return {
            id: product.id,
            columns: [{ text: product.name }, { text: product.description }],
          };
        })
      );
    });
  }, []);

  return (
    <section className="container left-aligned">
      <h1>Manage products</h1>
      <SelectTable
        header={productsList.header}
        rows={productsList.rows}
        button={productsList.button}
        outsideButtons={productsList.outsideButtons}
      />
    </section>
  );
}
