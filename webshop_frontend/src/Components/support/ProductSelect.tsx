type ProductSelectProps = {
  products: {
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
      <option key="0" value="0" disabled>
        Please choose a product
      </option>

      {products.map((product) => (
        <option key={product.name}>{product.name}</option>
      ))}
    </select>
  );
}
