#!/usr/bin/env python3

from sys import argv, exit
from typing import List, Optional, Any, Generator
import psycopg2

DATABASE_USERNAME = "postgres"
DATABASE_PASSWORD = "postgres"
DATABASE_HOST = "localhost"
DATABASE_PORT = 15432
DATABASE_NAME = "store"


DATABASE_URL = f"postgresql://{DATABASE_USERNAME}:{DATABASE_PASSWORD}@{DATABASE_HOST}:{DATABASE_PORT}/{DATABASE_NAME}"


class Customer(object):
    def __init__(self, id: str, email: str, created_at: str) -> None:
        self.id = id
        self.email = email
        self.password = "Password123!"
        self.created_at = created_at

    def __repr__(self) -> str:
        return f"{self.id};{self.email};{self.password};{self.created_at}"


class DbConnector(object):
    def __init__(self) -> None:
        self.__connection = psycopg2.connect(
            f"dbname='{DATABASE_NAME}' user='{DATABASE_USERNAME}' host='{DATABASE_HOST}' port='{DATABASE_PORT}' password='{DATABASE_PASSWORD}'"
        )
        self.__cursor: Optional[Any] = None

    def get_customers(self) -> Generator[Customer, None, None]:
        try:
            self.__cursor = self.__connection.cursor()
            self.__cursor.execute("SELECT id, email, created_at from customer")
            records = self.__cursor.fetchall()
            for record in records:
                yield Customer(record[0], record[1], record[2])
        finally:
            if self.__cursor:
                self.__cursor.close()


def main(args: List[str]) -> int:

    try:
        dbconn = DbConnector()
        for customer in dbconn.get_customers():
            print(customer)
    except Exception as e:
        print(f"failed to get customers: {e}")
        return 1

    return 0


if __name__ == "__main__":
    exit(main(argv[1:]))
