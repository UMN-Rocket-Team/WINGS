import { useParams } from "@solidjs/router";

const DummyComponent = () => {
    const params = useParams();

    return(
        <p>ID: {params.id}</p>
    );
};

export default DummyComponent;


