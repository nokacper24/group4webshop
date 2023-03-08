import { Link } from "react-router-dom";

export type ProductCardProps = {
  props: {
    product_id: string;
    name: string;
    description: string;
    sourceImage: string;
  };
};

export const ProductCard = ({ props: product }: ProductCardProps) => {
  return (
    <li>
      <Link className="product-card" to={`/products/${product.product_id}`}>
        <img
          height={200}
          width={300}
          src={product.sourceImage}
          alt={product.name}
          className="card-image"
        ></img>
        <h2>{product.name}</h2>
        <p className="card-description">{product.description}</p>
      </Link>
    </li>
  );
};
