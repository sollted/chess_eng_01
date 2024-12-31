import pstats
from pstats import SortKey

def analyze_profile(stats_file):
    # Create a Stats object
    p = pstats.Stats(stats_file)
    
    # Remove the long directory paths
    p.strip_dirs()
    
    print("TOP 20 FUNCTIONS BY CUMULATIVE TIME:")
    print("-" * 50)
    p.sort_stats(SortKey.CUMULATIVE).print_stats(20)
    
    """
    print("\nTOP 20 FUNCTIONS BY TOTAL TIME:")
    print("-" * 50)
    p.sort_stats(SortKey.TIME).print_stats(20)
    
    print("\nCALLERS OF TOP FUNCTIONS:")
    print("-" * 50)
    p.print_callers(20)
    """

analyze_profile('profile.stats')