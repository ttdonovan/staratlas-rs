import argparse
import duckdb
import streamlit as st
from streamlit_searchbox import st_searchbox

parser = argparse.ArgumentParser(description='Streamlit app with DuckDB connection.')
parser.add_argument('db_path', type=str, help='Path to the DuckDB database file')

args = parser.parse_args()

crew_duck = duckdb.connect(args.db_path, read_only=True)

def search_crew_by_name(name):
    query = '''
    SELECT *
    FROM crew
    WHERE name ilike '%' || $name || '%'
    '''
    values = crew_duck.execute(query, { "name": name }).fetchall()
    return values

st.set_page_config(layout="wide")
st.title("Star Atlas Crew")

st.write(search_crew_by_name("Anna Tolle"))

left, right = st.columns(2)
with left:
    crew1 = st_searchbox(search_crew_by_name,
        label="Crew 1",
        key="crew1_search",
        default="Anna Tolle",
        placeholder="Anna Tolle"
    )
with right:
    crew2 = st_searchbox(search_crew_by_name,
        label="Crew 2",
        key="crew2_search",
        default="Sammy Banx",
        placeholder="Sammy Banx"
    )

left, right = st.columns(2)
with left:
    st.write(crew1)
with right:
    st.write(crew2)