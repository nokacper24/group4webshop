import { useEffect, useState } from "react";
import { Product } from "../../../Interfaces";
import SelectTable, { SelectTableRowProps } from "../managing/SelectTable";

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
    return data.map((product: Product) => product);
  };

  useEffect(() => {
    fetchProducts().then((products) => {
      setProducts(
        products.map((product: Product) => {
          return {
            id: product.product_id,
            columns: [
              { text: product.display_name },
              { text: product.short_description },
            ],
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
