import { Link } from "react-router-dom";
import { Product } from "../../Interfaces";

export type ProductCardProps = {
  product: Product;
};

export const ProductCard = ({ product }: ProductCardProps) => {
  return (
    <li>
      <Link className="product-card" to={`/products/${product.product_id}`}>
        <img
          width={300}
          src={product.main_image}
          alt={product.display_name}
          className="card-image"
        ></img>
        <h2>{product.display_name}</h2>
        <p className="card-description">{product.short_description}</p>
      </Link>
    </li>
  );
};
