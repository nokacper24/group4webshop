type ProductSelectProps = {
  products: {
    id: string;
    name: string;
  }[];
};

/**
 * Represents the Product Select component.
 *
 * Contains a dropdown menu to select a product.
 *
 * @param products The list of products and their names.
 * @returns The ProductSelect component as a JSX element.
 */
export default function ProductSelect({ products }: ProductSelectProps) {
  return (
    <select id="product-select" name="products" defaultValue="0">
      <option value="0" disabled>
        Please choose a product
      </option>

      {products.map((product) => (
        <option key={product.id}>{product.name}</option>
      ))}
    </select>
  );
}
