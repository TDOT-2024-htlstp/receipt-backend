build version:
    docker build -t tdot-receipt-backend:{{version}} .

publish version: (build version)
    docker tag tdot-receipt-backend:{{version}} tobinio/tdot-receipt-backend:{{version}}
    docker push tobinio/tdot-receipt-backend:{{version}}