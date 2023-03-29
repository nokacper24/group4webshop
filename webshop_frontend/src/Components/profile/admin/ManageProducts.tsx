import { useEffect, useState } from "react";
import { Product } from "../../../Interfaces";
import SelectTable, {
  SelectTableProps,
  SelectTableRowProps,
} from "../managing/SelectTable";
import {
  createSelectTableProps,
  createRowProps,
} from "../managing/SelectTableFunctions";

export default function ManageProducts() {
  let baseUrl = import.meta.env.VITE_URL + ":" + import.meta.env.VITE_PORT;
  // Check if we are in production mode
  if (import.meta.env.PROD) {
    baseUrl = "";
  }

  const [products, setProducts] = useState<SelectTableRowProps[]>([]);

  const editProduct = (index: number) => {
    console.log("Editing: ", index); // TODO: Reroute to product page
  };

  const productsTable: SelectTableProps = createSelectTableProps(
    ["Product", "Description"],
    products,
    "Edit",
    editProduct,
    new Map([])
  );

  const fetchProducts = async () => {
    const response = await fetch(`${baseUrl}/api/products`);
    const data = await response.json();
    return data.map((product: Product) => product);
  };

  useEffect(() => {
    fetchProducts().then((products: Product[]) => {
      setProducts(
        products.map((product: Product) => {
          return createRowProps(product.product_id, [
            product.display_name,
            product.short_description,
          ]);
        })
      );
    });
  }, []);

  return (
    <section className="container left-aligned">
      <h1>Manage products</h1>
      <SelectTable
        header={productsTable.header}
        rows={productsTable.rows}
        button={productsTable.button}
        outsideButtons={productsTable.outsideButtons}
      />
    </section>
  );
}
