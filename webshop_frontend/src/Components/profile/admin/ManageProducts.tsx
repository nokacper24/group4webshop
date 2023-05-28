import { useEffect, useState } from "react";
import { Product } from "../../../Interfaces";
import SelectTable, {
  SelectTableProps,
  SelectTableRowProps,
} from "../select-table/SelectTable";
import {
  createSelectTableProps,
  createRowProps,
} from "../select-table/SelectTableFunctions";
import { fetchProducts } from "../../../ApiController";
import { useNavigate } from "react-router";
import { Link } from "react-router-dom";

export default function ManageProducts() {
  const navigate = useNavigate();

  const [products, setProducts] = useState<SelectTableRowProps[]>([]);

  const getProduct = (id: string) => {
    let product: SelectTableRowProps = {
      id: "",
      columns: [
        {
          text: "",
        },
      ],
    };

    products.forEach((p) => {
      if (p.id == id) {
        product = p;
      }
    });

    return product;
  };

  const editProduct = (id: string) => {
    navigate(`/product/manage/${getProduct(id).id}`);
  };

  const productsTable: SelectTableProps = createSelectTableProps(
    ["Product", "Description"],
    products,
    "Edit",
    editProduct,
    new Map([])
  );

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
      <Link className="default-button" to={"/product/create"}>Create new product</Link>
    </section>
  );
}
