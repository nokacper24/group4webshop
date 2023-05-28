import { LegacyRef, forwardRef } from "react";
import { Product } from "../../Interfaces";

type ProductSelectProps = {
  products: Product[];
};

/**
 * Represents the Product Select component.
 *
 * Contains a dropdown menu to select a product.
 *
 * @param products The list of products and their names.
 * @returns The ProductSelect component as a JSX element.
 */
const ProductSelect = forwardRef(function ProductSelect(
  { products }: ProductSelectProps,
  ref: LegacyRef<HTMLSelectElement>
) {
  return (
    <select ref={ref} id="product-select" name="products" defaultValue="0">
      <option value="0" disabled>
        Please choose a product
      </option>

      {products.map((product) => (
        <option key={product.product_id}>{product.display_name}</option>
      ))}
    </select>
  );
});

export default ProductSelect;
