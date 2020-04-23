"""
Implementation of vector-map solution described in README.md.
"""
def word_count_engine(document):
  """
  Create [token, count] list ordered by descending count
  first and document-order second.
  """
  tokens = []
  for t in document.split():
    ct = clean_token(t)
    if ct:
      tokens.append(ct)

  counts = {}
  for t in tokens:
    if t in counts:
      counts[t] += 1
    else:
        counts[t] = 1
  ordering = [[] for _ in range(max(counts.values())+1)]
  for t in tokens:
    if t in counts:
      ordering[counts[t]].append(t)
      del counts[t]
  result = []
  for i, olist in enumerate(reversed(ordering)):
    count = len(ordering) - i -1
    for word in olist:
      result.append([word, str(count)])
  return result


def clean_token(token):
  """
  Takes whitespace split token and produces lower case
  stripped of punctuation
  """
  return ''.join([c for c in token.lower() if c >= 'a' and c <= 'z'])
document = "Practice makes perfect. you'll only get Perfect by practice. just practice!"



print(word_count_engine(document))
