from typing import List, Dict, Any

from fetchers import mitre_fetcher
from fetchers import detection_fetcher

class ThreatContentFetcher:
    """
    Orchestrates fetching threat content from various sources,
    including MITRE ATT&CK and detection content (SPIRES, DSL, ML Training).
    """

    def __init__(self, config: Dict[str, Any]):
        """
        Initializes the ThreatContentFetcher with a configuration.

        Args:
            config (Dict[str, Any]): Configuration dictionary containing settings for fetchers.
                                     Expected keys include 'mitre', 'detection', etc.
        """
        self.config = config

        # Initialize fetchers
        self.mitre_fetcher = mitre_fetcher.MitreFetcher(config.get("mitre", {}))
        self.detection_fetcher = detection_fetcher.DetectionFetcher(config.get("detection", {}))


    def fetch_mitre_attack(self) -> List[Dict[str, Any]]:
        """
        Fetches MITRE ATT&CK data.

        Returns:
            List[Dict[str, Any]]: A list of dictionaries representing MITRE ATT&CK techniques and tactics.
        """
        return self.mitre_fetcher.fetch_data()  # Assuming MitreFetcher has a fetch_data method

    def fetch_detection_content(self, content_type: str) -> List[Dict[str, Any]]:
        """
        Fetches detection content based on the specified type (SPIRES, DSL, ML Training).

        Args:
            content_type (str): The type of detection content to fetch ("SPIRES", "DSL", "ML Training").

        Returns:
            List[Dict[str, Any]]: A list of dictionaries representing the fetched detection content.

        Raises:
            ValueError: If an invalid content_type is provided.
        """
        if content_type not in ["SPIRES", "DSL", "ML Training"]:
            raise ValueError(f"Invalid content_type: {content_type}.  Must be SPIRES, DSL, or ML Training.")

        return self.detection_fetcher.fetch_data(content_type)  # Assuming DetectionFetcher has a fetch_data method

    def fetch_all(self) -> Dict[str, List[Dict[str, Any]]]:
        """
        Fetches all threat content, including MITRE ATT&CK and all detection content types.

        Returns:
            Dict[str, List[Dict[str, Any]]]: A dictionary containing fetched data, keyed by content type
            (e.g., "mitre", "SPIRES", "DSL", "ML Training").
        """
        all_data = {
            "mitre": self.fetch_mitre_attack(),
            "SPIRES": self.fetch_detection_content("SPIRES"),
            "DSL": self.fetch_detection_content("DSL"),
            "ML Training": self.fetch_detection_content("ML Training"),
        }
        return all_data

# Example Usage (Illustrative - replace with actual configuration)
if __name__ == '__main__':
    # Sample Configuration (replace with your actual config)
    config = {
        "mitre": {
            "source": "some_mitre_data_source" # Replace with actual source config
        },
        "detection": {
            "spires_source": "some_spires_data_source", # Replace with actual source configs
            "dsl_source": "some_dsl_data_source",
            "ml_training_source": "some_ml_training_data_source",
        }
    }

    fetcher = ThreatContentFetcher(config)

    # Fetch MITRE data
    mitre_data = fetcher.fetch_mitre_attack()
    print(f"Fetched {len(mitre_data)} MITRE records.")
    # Optionally print some of the data:
    if mitre_data:
      print(f"Example MITRE Technique Name: {mitre_data[0].get('name', 'N/A')}")

    # Fetch SPIRES data
    spires_data = fetcher.fetch_detection_content("SPIRES")
    print(f"Fetched {len(spires_data)} SPIRES records.")
    if spires_data:
      print(f"Example SPIRES rule: {spires_data[0].get('rule', 'N/A')}")



    # Fetch all data
    all_data = fetcher.fetch_all()
    print(f"Fetched all data: {all_data.keys()}")