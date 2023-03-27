#!/usr/bin/env python3

from datetime import datetime
from os.path import dirname
from random import choice
from sys import argv, exit
from typing import List, Tuple, Optional, Any
from uuid import uuid4
import csv
import getopt
import hashlib
import psycopg2

DATABASE_USERNAME = "postgres"
DATABASE_PASSWORD = "postgres"
DATABASE_HOST = "localhost"
DATABASE_PORT = 15432
DATABASE_NAME = "store"

DATABASE_URL = f"postgresql://{DATABASE_USERNAME}:{DATABASE_PASSWORD}@{DATABASE_HOST}:{DATABASE_PORT}/{DATABASE_NAME}"

TOOLS_DIR = dirname(__file__)
ARTICLES_CSV = f"{TOOLS_DIR}/articles.csv"
CUSTOMER_CSV = f"{TOOLS_DIR}/customers.csv"

CustomerList = List[Tuple[str, str]]


class Article(object):
    def __init__(self, name: str, description: str) -> None:
        self.id = str(uuid4())
        self.name = name
        self.description = description
        self.unit_price = f"{choice(range(0, 100))}.{choice(range(0, 99))}"


class Customer(object):
    def __init__(self, email: str, password: str) -> None:
        digest = hashlib.sha256()
        digest.update(password.encode("utf-8"))
        self.id = str(uuid4())
        self.email = email
        self.password = digest.hexdigest()
        self.created_at = datetime.now()


class OrderedArticle(object):
    def __init__(self, article: Article, quantity: int) -> None:
        self.id = str(uuid4())
        self.article = article
        self.quantity = quantity


class Order(object):
    def __init__(self, customer_id: str, articles: List[OrderedArticle]) -> None:
        self.id = str(uuid4())
        self.customer_id = customer_id
        self.created_at = datetime.now()
        self.articles = articles


def parse_articles_csv() -> List[Article]:
    articles = []
    with open(ARTICLES_CSV) as csv_file:
        reader = csv.reader(csv_file, delimiter=";")
        for line in reader:
            articles.append(Article(line[0], line[1]))
    return articles


def parse_customers_csv() -> CustomerList:
    customers = []
    with open(CUSTOMER_CSV) as csv_file:
        reader = csv.reader(csv_file, delimiter=";")
        for line in reader:
            customers.append((line[0], line[1]))
    return customers


def generate_customers_from_names_list(
    names_list: CustomerList, amount: int
) -> List[Customer]:
    customers = []
    names = list(map(lambda x: x[0], names_list))
    surnames = list(map(lambda x: x[1], names_list))
    for _ in range(0, amount):
        name = choice(names)
        surname = choice(surnames)
        year = choice(range(1950, 2005))
        email = f"{name.lower()}.{surname.lower()}{year}@gmail.com"
        password = "Password123!"
        customers.append(Customer(email, password))
    return customers


def generate_order(
    articles: List[Article], customer: Customer, articles_per_order: int
) -> Order:
    order_articles = []
    for _ in range(0, articles_per_order):
        quantity = choice(range(1, 5))
        order_articles.append(OrderedArticle(choice(articles), quantity))

    return Order(customer.id, order_articles)


# database


class DbConnector(object):
    def __init__(self) -> None:
        self.__connection = psycopg2.connect(
            f"dbname='{DATABASE_NAME}' user='{DATABASE_USERNAME}' host='{DATABASE_HOST}' port='{DATABASE_PORT}' password='{DATABASE_PASSWORD}'"
        )
        self.__cursor: Optional[Any] = None

    def begin(self):
        self.__cursor = self.__connection.cursor()
        self.__cursor.execute("BEGIN;")

    def commit(self):
        if self.__cursor is not None:
            self.__cursor.execute("COMMIT;")
            self.__cursor = None
        else:
            raise Exception("transaction must be started first")

    def rollback(self):
        if self.__cursor is not None:
            self.__cursor.execute("ROLLBACK;")
            self.__cursor = None
        else:
            raise Exception("transaction must be started first")

    def reset(self):
        try:
            self.begin()
            if self.__cursor:
                self.__cursor.execute("DELETE FROM order_article")
                self.__cursor.execute("DELETE FROM customer_order")
                self.__cursor.execute("DELETE FROM article")
                self.__cursor.execute("DELETE FROM customer")

        except Exception as e:
            self.rollback()
            raise e

        self.commit()

    def insert_customer(self, customer: Customer):
        if self.__cursor is not None:
            query = f"INSERT INTO customer (id, email, password, created_at) VALUES ('{customer.id}', '{customer.email}', '{customer.password}', '{self.__timestamp(customer.created_at)}')"
            print(query)
            self.__cursor.execute(query)
        else:
            raise Exception("transaction must be started first")

    def insert_article(self, article: Article):
        if self.__cursor is not None:
            query = f"INSERT INTO article (id, name, description, unit_price) VALUES ('{article.id}', '{article.name}', '{article.description}', '{article.unit_price}')"
            print(query)
            self.__cursor.execute(query)
        else:
            raise Exception("transaction must be started first")

    def insert_order(self, order: Order):
        if self.__cursor is not None:
            query = f"INSERT INTO customer_order (id, customer_id, created_at, status) VALUES ('{order.id}', '{order.customer_id}', '{self.__timestamp(order.created_at)}', 'created')"
            print(query)
            self.__cursor.execute(query)
            for article in order.articles:
                self.__insert_order_article(order.id, article)
        else:
            raise Exception("transaction must be started first")

    def __insert_order_article(self, order_id: str, order_article: OrderedArticle):
        if self.__cursor is not None:
            query = f"INSERT INTO order_article (id, quantity, unit_price, order_id, article_id) VALUES ('{order_article.id}', '{order_article.quantity}', '{order_article.article.unit_price}', '{order_id}', '{order_article.article.id}')"
            print(query)
            self.__cursor.execute(query)
        else:
            raise Exception("transaction must be started first")

    def __timestamp(self, timestamp: datetime) -> str:
        return datetime.strftime(timestamp, "%Y-%m-%d %H:%M:%S")


def main(args: List[str]) -> int:
    # parse opts
    customers = 20
    orders_per_customer = 3
    articles_per_order = 3
    reset = False

    options, args = getopt.getopt(
        args, "", ["customers=", "orders-per-customer=", "articles-per-order=", "reset"]
    )
    for opt, arg in options:
        if opt == "--customers":
            customers = int(arg)
        elif opt == "--orders-per-customer":
            orders_per_customer = int(arg)
        elif opt == "--articles-per-order":
            articles_per_order = int(arg)
        elif opt == "--reset":
            reset = True

    # init database
    try:
        database = DbConnector()
    except Exception as e:
        print(e)
        return 1
    # reset db
    if reset:
        print("db reset...")
        try:
            database.reset()
        except Exception as e:
            print(f"failed to reset db: {e}")
            return 1
        print("reset OK")
    # parse data
    try:
        articles = parse_articles_csv()
    except Exception as e:
        print(f"failed to parse articles csv: {e}")
        return 1
    try:
        customers = generate_customers_from_names_list(parse_customers_csv(), customers)
    except Exception as e:
        print(f"failed to parse customers csv: {e}")
        return 1

    # start tx
    try:
        database.begin()
        for article in articles:
            database.insert_article(article)
        for customer in customers:
            database.insert_customer(customer)
            # generate orders per customer
            for _ in range(0, orders_per_customer):
                order = generate_order(articles, customer, articles_per_order)
                database.insert_order(order)
        database.commit()
    except Exception as e:
        print(f"transaction failed: {e}")
        database.rollback()
        return 1

    return 0


if __name__ == "__main__":
    exit(main(argv[1:]))
