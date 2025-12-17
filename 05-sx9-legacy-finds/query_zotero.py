import sqlite3
import os

DB_PATH = "/Users/cp5337/Zotero/zotero.sqlite"

def query_zotero():
    if not os.path.exists(DB_PATH):
        print(f"❌ Database not found at {DB_PATH}")
        return

    try:
        conn = sqlite3.connect(DB_PATH)
        cursor = conn.cursor()
        
        # Query for items with relevant keywords in title or abstract
        query = """
        SELECT 
            items.key,
            itemDataValues.value AS title
        FROM items
        JOIN itemData ON items.itemID = itemData.itemID
        JOIN itemDataValues ON itemData.valueID = itemDataValues.valueID
        WHERE itemData.fieldID = 1  -- Title field
        AND (
            itemDataValues.value LIKE '%NASA%' OR
            itemDataValues.value LIKE '%Mission Operations%' OR
            itemDataValues.value LIKE '%Voice%' OR
            itemDataValues.value LIKE '%Interface%' OR
            itemDataValues.value LIKE '%HCI%'
        )
        LIMIT 20;
        """
        
        cursor.execute(query)
        results = cursor.fetchall()
        
        print(f"🔍 Found {len(results)} relevant papers:\n")
        for key, title in results:
            print(f"- {title} (Key: {key})")
            
        conn.close()
        
    except Exception as e:
        print(f"❌ Error querying database: {e}")

if __name__ == "__main__":
    query_zotero()
